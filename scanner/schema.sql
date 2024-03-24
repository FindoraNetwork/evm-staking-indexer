drop table if exists evm_last_height;
drop table if exists evm_stakes;
drop table if exists evm_delegations;
drop table if exists evm_undelegations;
drop table if exists evm_jailed;
drop table if exists evm_punish;
drop table if exists evm_update_validator;
drop table if exists evm_coinbase_mint;
drop table if exists evm_receipts;
drop table if exists evm_audit;
drop table if exists evm_validators;

create table if not exists evm_last_height(
    tip varchar(3) not null,
    height bigint not null,
    primary key(tip)
);

create table if not exists evm_stakes(
    tx_id varchar(66) not null,
    block_id varchar(66) not null,
    block_num bigint not null,
    tm timestamp not null,
    validator varchar(66) not null,
    pubkey varchar(66) not null,
    ty integer not null,
    staker varchar(66) not null,
    amount numeric(48) not null,
    memo jsonb not null,
    rate numeric(48) not null,
    primary key(tx_id,validator,staker)
);

create table if not exists evm_delegations(
    tx_id varchar(66) not null,
    block_id varchar(66) not null,
    block_num bigint not null,
    tm timestamp not null,
    validator varchar(66) not null,
    delegator varchar(66) not null,
    amount numeric(48) not null,
    primary key(tx_id,validator,delegator)
);

create table if not exists evm_undelegations(
    tx_id varchar(66) not null,
    block_id varchar(66) not null,
    block_num bigint not null,
    tm timestamp not null,
    idx bigint not null,
    validator varchar(66) not null,
    delegator varchar(66) not null,
    unlock_time bigint not null,
    amount numeric(48) not null,
    op_type integer not null,
    primary key(tx_id,validator,delegator)
);

create table if not exists evm_jailed(
    tx_id varchar(66) not null,
    block_id varchar(66) not null,
    block_num bigint not null,
    tm timestamp not null,
    validator varchar(66) not null,
    jailed boolean not null,
    primary key(tx_id,validator)
);

create table if not exists evm_punish(
    tx_id varchar(66) not null,
    block_id varchar(66) not null,
    block_num bigint not null,
    tm timestamp not null,
    voted jsonb not null,
    unvoted jsonb not null,
    byztine jsonb not null,
    primary key(tx_id)
);

create table if not exists evm_update_validator(
    tx_id varchar(66) not null,
    block_id varchar(66) not null,
    block_num bigint not null,
    tm timestamp not null,
    validator varchar(66) not null,
    memo jsonb not null,
    rate numeric(48) not null,
    primary key(tx_id,validator)
);

create table if not exists evm_coinbase_mint(
    tx_id varchar(66) not null,
    block_id varchar(66) not null,
    block_num bigint not null,
    tm timestamp not null,
    validator varchar(66) not null,
    delegator varchar(66) not null,
    pubkey varchar(66) not null,
    amount numeric(48) not null,
    primary key(tx_id,validator,delegator)
);

create table if not exists evm_receipts(
    tx_id varchar(66) not null,
    block_id varchar(66) not null,
    block_num bigint not null,
    from_addr varchar(66) not null,
    to_addr varchar(66) not null,
    tm timestamp not null,
    value jsonb not null,
    primary key(tx_id)
);

create index idxerbid on evm_receipts(block_id);
create index idxerbn on evm_receipts(block_num);
create index idxerfrom on evm_receipts(from_addr);
create index idxerto on evm_receipts(to_addr);

create table if not exists evm_audit(
    tx_id varchar(66) not null,
    block_num bigint not null,
    validator varchar(66) not null,
    delegator varchar(66) not null,
    amount numeric(48) not null,
    op integer not null
);

create table if not exists evm_validators(
    block_num bigint not null,
    validator varchar(66) not null,
    pubkey varchar(66) not null,
    pubkey_type integer not null,
    rate numeric(48) not null,
    staker varchar(66) not null,
    power numeric(48) not null,
    unbound numeric(48) not null,
    punish_rate numeric(48) not null,
    begin_block bigint not null,
    active boolean not null,
    jailed boolean not null,
    unjail_time timestamp not null,
    should_vote integer not null,
    voted integer not null,
    primary key (block_num,validator,staker)
) partition by hash(block_num);

create index idxevbn on evm_validators(block_num);
create index idxevvld on evm_validators(validator);

create table evm_validators_0 partition of evm_validators for values with (MODULUS 5, REMAINDER 0);
create table evm_validators_1 partition of evm_validators for values with (MODULUS 5, REMAINDER 1);
create table evm_validators_2 partition of evm_validators for values with (MODULUS 5, REMAINDER 2);
create table evm_validators_3 partition of evm_validators for values with (MODULUS 5, REMAINDER 3);
create table evm_validators_4 partition of evm_validators for values with (MODULUS 5, REMAINDER 4);

create or replace function put_delegate() returns trigger as $$
    begin
        insert into evm_audit select new.tx_id,new.block_num,new.validator,new.delegator,new.amount,0;
        return null;
    end;
$$ language plpgsql;

create trigger delegate_trigger
    after insert or update
    on evm_delegations
    for each row execute procedure put_delegate();

create or replace function put_undelegate() returns trigger as $$
    begin
        insert into evm_audit select new.tx_id,new.block_num,new.validator,new.delegator,-new.amount,1;
        return null;
    end;
$$ language plpgsql;

create trigger undelegate_trigger
    after insert or update
    on evm_undelegations
    for each row execute procedure put_undelegate();

create or replace function update_validator() returns trigger as $$
    begin
        update evm_stakes set memo=new.memo,rate=new.rate where validator=new.validator;
        return null;
    end;
$$ language plpgsql;

create trigger update_validator_trigger
    after insert or update
    on evm_update_validator
    for each row execute procedure update_validator();


