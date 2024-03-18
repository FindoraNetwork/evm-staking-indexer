# API
### `GET /api/validator/list`
* 功能：获取当前validator集合
* 无参数
* Request: `http://localhost/api/validator/list`
* Response:
```json
  [
    {
      "address": "400c0f623f71c8bfeb6b4ec71b54624925c1a6c6",
      "power": "235520707012010263794755",
      "public_key": "0xc3239e8c68ffa29572d22d7de7ddbad7d434706b19d13318e01d7ff216a6593f",
      "public_key_type": 2
    },
    {
      "address": "ccac2728809428d8d2967b3d9093d6a989df072a",
      "power": "665619655200714635792002",
      "public_key": "0xf7299c12de8a267366b86a67b6b9465486a8ad17b6db2a23ee85824936342999",
      "public_key_type": 2
    },
    {
      "address": "9aee1a3ab861102f3039a0f2a55b89614f1968d8",
      "power": "266499680081979530729436",
      "public_key": "0x83a4307116025fa20474156bfcffe9d5fe324bf208971b96b567828f1fff9f62",
      "public_key_type": 2
    }
  ]
```

### `GET /api/validator/detail`
* 功能：获取validator的详细信息
* 参数

| 参数        | 类型     | 必传 | 说明          |
|-----------|--------|----|-------------|
| validator | string | Y  | validator地址 |

* Request: `http://localhost/api/validator/detail?validator=400c0f623f71c8bfeb6b4ec71b54624925c1a6c6`  
* Response:
  ```json
  {
    "public_key": "0xc3239e8c68ffa29572d22d7de7ddbad7d434706b19d13318e01d7ff216a6593f",
    "public_key_type": 2,
    "rate": "500000",
    "staker": "0x9407580f89e7ade491844bb387cae4bf71c40d6f",
    "power": "235520707012010263794755",
    "total_unbound_amount": "0",
    "punish_rate": "999959540818290156",
    "begin_block": "4636000"
  }
  ```

### `GET /api/validator/status`
* 功能：获取validator的状态
*  参数

| 参数        | 类型     | 必传 | 说明          |
|-----------|--------|----|-------------|
| validator | string | Y  | validator地址 |

* Request: `http://localhost/api/validator/status?validator=400c0f623f71c8bfeb6b4ec71b54624925c1a6c6`
* Response:
```json
  {
    "heap_index_off1": "1",
    "is_active": true,
    "jailed": false,
    "unjail_datetime": 1697279406,
    "should_vote": 977,
    "voted": 974
  }
```
### `GET /api/claims`
* 功能：获取地址提取奖励记录，若不指定delegator，则返回任意地址的提取记录
* 参数

| 参数        | 类型     | 必传 | 说明          |
|-----------|--------|----|-------------|
| delegator | string | N  | delegator地址 |
| page      | number | N  | 页码，默认1      |
| page_size | number | N  | 页大小，默认10    |

* Request:`http://localhost/api/claims?delegator=2d15d52cc138ffb322b732239cd3630735abac88&page=1&page_size=10`
* Response:
```json
{
"total": 6,
"page": 1,
"page_size": 3,
"data": [
    {
      "tx": "281d962bdef2919032dfd704c9babffda20549e5ba58a3ac2cba2802476f91e7",
      "block_num": 4731288,
      "validator": "000e33ab7471186f3b1de9fc08bb9c480f453590",
      "delegator": "2d15d52cc138ffb322b732239cd3630735abac88",
      "amount": "697734392375302244"
    },
    {
      "tx": "281d962bdef2919032dfd704c9babffda20549e5ba58a3ac2cba2802476f91e7",
      "block_num": 4731288,
      "validator": "00121f5cfd8d95f8c194ed4ccff47bbd1904b791",
      "delegator": "2d15d52cc138ffb322b732239cd3630735abac88",
      "amount": "34242028226633"
    },
    {
      "tx": "281d962bdef2919032dfd704c9babffda20549e5ba58a3ac2cba2802476f91e7",
      "block_num": 4731288,
      "validator": "09ef1db6b67d1cbf7eba6bd9b204611848993df7",
      "delegator": "2d15d52cc138ffb322b732239cd3630735abac88",
      "amount": "1064514338839250640"
    }
  ]
}
```

### `GET /api/delegations`
* 功能：获取地址的delegate记录，若不指定delegator，则返回任意地址的记录
* 参数

| 参数        | 类型     | 必传 | 说明          |
|-----------|--------|----|-------------|
| delegator | string | N  | delegator地址 |
| page      | number | N  | 页码，默认1      |
| page_size | number | N  | 页大小，默认10    |

* Request: `http://localhost/api/delegations?delegator=2d15d52cc138ffb322b732239cd3630735abac88&page=1&page_size=10`  
* Response:
```json
{
  "total": 2,
  "page": 1,
  "page_size": 10,
  "data": [
    {
      "tx": "b98281e3ad708023bcf67f384289361ed07568447fcfc25e9184482e04fe81c9",
      "block_num": 4709313,
      "validator": "2a75d9238dbbf14891f7bffbba7ef86ca0e98cc9",
      "delegator": "2d15d52cc138ffb322b732239cd3630735abac88",
      "amount": "3000000000000000000"
    },
    {
      "tx": "70febc1d66c96ca3c9207c2a0cdefc4558354107fc9a2767aa4493f18362efee",
      "block_num": 4704494,
      "validator": "d518c4f95a3f39ed853a2614566897c4ad5a008f",
      "delegator": "2d15d52cc138ffb322b732239cd3630735abac88",
      "amount": "1000000000000000000"
    }
  ]
}
```

### `GET /api/undelegations`
* 功能：获取地址的undelegate记录，若不指定delegator，则返回任意地址的记录
* 参数

| 参数        | 类型     | 必传 | 说明          |
|-----------|--------|----|-------------|
| delegator | string | N  | delegator地址 |
| page      | number | N  | 页码，默认1      |
| page_size | number | N  | 页大小，默认10    |

* Request: `http://localhost/api/undelegations?delegator=836a7c6e4ec6399365d4f27aefeb277345e2a655&page=1&page_size=10`
* Response:
```json
{
  "total": 2,
  "page": 1,
  "page_size": 10,
  "data": [
    {
      "tx": "21c4c1d0870d9c12b0cde438fc51d1edce987c0937509d858c2930e2d3ed30a8",
      "block_num": 4718062,
      "index": 96,
      "validator": "3560fd0632b4e2f4f16490bbd9cd0a763045bf35",
      "delegator": "836a7c6e4ec6399365d4f27aefeb277345e2a655",
      "unlock_time": 1697106621,
      "amount": "999926920000000000000",
      "op_type": 0
    },
    {
      "tx": "6e814c6f13030a84c9f4d24caa0ea7db1d3a7fe2f50334ca6629d04d0a4b12ff",
      "block_num": 4718018,
      "index": 56,
      "validator": "69e2b6c4c1122172e69af48e0aec36b7f7c8005a",
      "delegator": "836a7c6e4ec6399365d4f27aefeb277345e2a655",
      "unlock_time": 1696386941,
      "amount": "4997408848000000000000",
      "op_type": 0
    }
  ]
}
```

### `GET /api/delegators`
* 功能：获取质押到该validator的delegator集合
* 参数

| 参数        | 类型     | 必传 | 说明          |
|-----------|--------|----|-------------|
| validator | string | Y  | validator地址 |
| page      | number | N  | 页码，默认1      |
| page_size | number | N  | 页大小，默认10    |

* Request:`http://localhost/api/delegators?validator=09ef1db6b67d1cbf7eba6bd9b204611848993df7&page=1&page_size=20`
* Response:
```json
  {
  "total": 411,
  "page": 1,
  "page_size": 20,
  "data": [
    "1c33fe72bee0599087d33940bc70c246e4897c17",
    "ee4ab7c2214e4059e6c510977d248b821c9139d2",
    "c5409347b0c3ef70d8a964ca286e9d7b4c66c49e",
    "7380f6c99ae40ece405fd5b55b4405ca5ab8bc4a",
    "fc34af86793be1686b087c61ec4dfc08e11ab377",
    "f9787a5dfb0c02432374fec2c033b1011756c2d0",
    "7f4c41775305853a3fe80e607b27659534cc3f48",
    "6751c301ac6333dacb6a1dbdb5dc5d6d9d31de4e",
    "e6032af5acd363377be84efc55b34878a245c5cd",
    "3eb961e5902a5c8eb197091089bce2a9114b4d71",
    "63d60979f55740412e1a11dae70c393bebc25baf",
    "f8d379993517b51a59c6cc9badfa959411f65182",
    "585d29ebf14c2bcf0ebd0df1aadab3fb7c0a7d1f",
    "fc7e9b7b1d62bf06a99bd5bb61e1119950ea7f15",
    "f61bdcee6a66a66cfa2a3b0c952e38968575fddd",
    "fc6c6487c4499a22107df5a4aaf9d6d2607405d3",
    "7df23486f82e4669051ace292cf5cb69f0808304",
    "3513f4ed28af9a2f16acb80966abaff80262b54e",
    "023d0ba3582b37fb973d8c56894ab3507e4551a9",
    "352c847ecfd3b326bca8c2c6e33fcddd6cf5c0fd"
  ]
}
```

### `GET /api/validators`
* 功能：获取delegator所质押的validator集合
* 参数

| 参数        | 类型     | 必传 | 说明          |
|-----------|--------|----|-------------|
| delegator | string | Y  | delegator地址 |
| page      | number | N  | 页码，默认1      |
| page_size | number | N  | 页大小，默认10    |

* Request:`http://localhost/api/validators?delegator=2c6af585b24a93912676f797b30dbc1b8f654ce4&page=1&page_size=20`
* Response:
```json
{
  "total": 17,
  "page": 1,
  "page_size": 20,
  "data": [
    "000e33ab7471186f3b1de9fc08bb9c480f453590",
    "09ef1db6b67d1cbf7eba6bd9b204611848993df7",
    "2440346158429ceae65c15121d0c40560820cfc2",
    "26aa7581263332f47e0ce17cf4b1f34d22c7f4cb",
    "2a75d9238dbbf14891f7bffbba7ef86ca0e98cc9",
    "4e3da3856567e4ab21b70c25fb7c19729fceebca",
    "544fec0d957816c880f1ac4c4ca239feede0ac70",
    "54937e208cf724f06ca723173c54fc5e8f9ad01a",
    "55dbb6b98e70f4a9905c880b7c66282b5d5ad000",
    "629f2d3da692107bfc5db3122c44fcfaa72db8c7",
    "7efe6655436794be8720d0b0efdffdc2a8bff9e4",
    "805b1f87212164fd1db64b8ed63a8f2c42aac647",
    "d518c4f95a3f39ed853a2614566897c4ad5a008f",
    "e012aa66c83999e3862c8aa534b9ce66fc14a37a",
    "e8f6748439da597a43ed150f55f6b48e30494bd6",
    "eac5792572eb726aa0dba9a7afa9757f8063c6c9",
    "fd8c65634a9d8899fa14200177af19d24f6e1c37"
  ]
}
```


### `GET /api/bound`
* 功能：获取delegator的bound和unboud数量
* 参数

| 参数        | 类型     | 必传 | 说明          |
|-----------|--------|----|-------------|
| validator | string | Y  | validator地址 |
| delegator | string | Y  | delegator地址 |

* Request:`http://localhost/api/bound?validator=09ef1db6b67d1cbf7eba6bd9b204611848993df7&delegator=2d15d52cc138ffb322b732239cd3630735abac88`
* Response:
```json
  {
    "bound_amount": "110000001800000063120",
    "unbound_amount": "0"
  }
```

### `GET /api/reward`
* 功能：获取delegator的奖励数量
* 参数

| 参数        | 类型     | 必传 | 说明          |
|-----------|--------|----|-------------|
| delegator | string | Y  | delegator地址 |

* Request:`http://localhost/api/reward?delegator=2d15d52cc138ffb322b732239cd3630735abac88`
* Response:
```json
{
  "reward": "16742332457649244907"
}
```

### `GET /api/debt`
* 功能：获取validator对delegator的奖励负债
* 参数

| 参数        | 类型     | 必传 | 说明          |
|-----------|--------|----|-------------|
| validator | string | Y  | validator地址 |
| delegator | string | Y  | delegator地址 |

* Request:`http://localhost/api/debt?validator=d518c4f95a3f39ed853a2614566897c4ad5a008f&delegator=2d15d52cc138ffb322b732239cd3630735abac88`
* Response:
```json
{
  "debt": "96558069283467635"
}
```

### `GET /api/sum`
* 功能：获取delegator的delegate, undelegate, claim总量
* 参数

| 参数        | 类型     | 必传 | 说明          |
|-----------|--------|----|-------------|
| delegator | string | Y  | delegator地址 |

* Request: `http://localhost/api/sum?delegator=2d15d52cc138ffb322b732239cd3630735abac88`
* Response:
```json
{
  "sum_delegate": "86000000000000000000",
  "sum_undelegate": "467980000000000000000",
  "sum_claim": "1818488527919773610"
}
```