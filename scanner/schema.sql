create table if not exists evm_last_height(
    tip varchar(3) not null,
    height bigint not null,
    primary key(tip)
);

create table if not exists evm_e_stake(
    tx varchar(66) not null,
    block_num bigint not null,
    validator varchar(66) not null,
    public_key varchar(66) not null,
    ty integer not null,
    staker varchar(66) not null,
    amount numeric(48) not null,
    memo text,
    rate numeric(48) not null,
    primary key(tx,validator,staker)
);

create table if not exists evm_e_delegation(
    tx varchar(66) not null,
    block_num bigint not null,
    validator varchar(66) not null,
    delegator varchar(66) not null,
    amount numeric(48) not null,
    primary key(tx,validator,delegator)
);

create table if not exists evm_e_undelegation(
    tx varchar(66) not null,
    block_num bigint not null,
    idx bigint not null,
    validator varchar(66) not null,
    delegator varchar(66) not null,
    unlock_time bigint not null,
    amount numeric(48) not null,
    op_type integer not null,
    primary key(tx,validator,delegator)
);

create table if not exists evm_e_proposer(
    tx varchar(66) not null,
    block_num bigint not null,
    proposer varchar(66) not null,
    primary key(tx,proposer)
);

create table if not exists evm_e_epoch(
    tx varchar(66) not null,
    block_num bigint not null,
    epoch bigint not null,
    primary key(tx,epoch)
);

create table if not exists evm_e_jailed(
    tx varchar(66) not null,
    block_num bigint not null,
    validator varchar(66) not null,
    jailed boolean not null,
    primary key(tx,validator)
);

create table if not exists evm_e_punish(
    tx varchar(66) not null,
    block_num bigint not null,
    voted jsonb not null,
    unvoted jsonb not null,
    byztine jsonb not null,
    primary key(tx)
);

create table if not exists evm_e_update_validator(
    tx varchar(66) not null,
    block_num bigint not null,
    validator varchar(66) not null,
    memo text,
    rate numeric(48) not null,
    primary key(tx,validator)
);

create table if not exists evm_e_coinbase_mint(
    tx varchar(66) not null,
    block_num bigint not null,
    validator varchar(66) not null,
    delegator varchar(66) not null,
    pubkey varchar(66) not null,
    amount numeric(48) not null,
    primary key(tx,validator,delegator)
);
