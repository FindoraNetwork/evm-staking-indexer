# Evm Staking Indexer API Spec


## [Validator](#1)
* [1.1 获取Validator集合](#1.1)
* [1.2 获取validator最近20笔质押变化](#1.2)
* [1.3 获取validator的delegate记录](#1.3)
* [1.4 获取validator的undelegate记录](#1.4)
* [1.5 获取validator最近10日vote变化](#1.5)
* [1.6 获取质押到validator的delegator集合](#1.6)
* [1.7 获取delegator所质押的validator集合](#1.7)
* [1.8 获取地址（delegator）的delegate记录](#1.8)
* [1.9 获取地址（delegator）的undelegate记录](#1.9)


## [Contract](#2)
* [2.1 获取bound数量](#2.1)
* [2.2 获取reward数量](#2.2)
* [2.3 获取debt数量](#2.3)
* [2.4 获取validator数据](#2.4)
* [2.5 获取validator状态](#2.5)


## [Other](#3)
* [3.1 统计delegate,undelegate,claim总量](#3.1)


<h3 id="1.1">1.1 获取Validator集合</h3>

* `GET /api/valiators`
* 参数 

| 参数        | 类型     | 必传 | 说明          |
|-----------|--------|----|-------------|
| validator | string | N  | validator地址 |
| online    | bool   | N  | 过滤active    |
| offline   | bool   | N  | 过滤jailed    |
| page      | number | N  | 页码，默认1      |
| page_size | number | N  | 页大小，默认10    |

* Request: 
  * 查询active为true的validator: `http://localhost/api/validators?online=true&page=1&page_size=5`
  * 查询单个validator: `http://localhost/api/validators?validator=0xd518c4f95a3f39ed853a2614566897c4ad5a008f`
* Response: 
  * 返回结果按power（质押总量）降序排列 
```json
{
  "total": 31,
  "page": 1,
  "page_size": 5,
  "data": [{
    "validator": "0x09ef1db6b67d1cbf7eba6bd9b204611848993df7",
    "staker": "0x307836383239e280a638326564",
    "active": true,
    "jailed": false,
    "should_vote": 710,
    "voted": 710,
    "pubkey": "0xbee5782b5b004b4176e7991cec5819c4aebfae63ff9cc250dea602df3a8c382f",
    "pubkey_type": 2,
    "rate": "0",
    "power": "152878110978597052092473766",
    "unbound_amount": "152807356870999999999998",
    "punish_rate": "999999870000007789",
    "begin_block": 4636000,
    "unjail_time": 0,
    "memo": {
      "desc": "ACEcryptory, the ACE node operator! 0% fee for the first 10 epochs !",
      "logo": "https://drive.google.com/file/d/18blSXpl7KxtzzUWZlC6qsyEk2X13UoNo/view?usp=sharing",
      "name": "ACEcryptory",
      "website": "http://www.acecryptory.io/"
    }
  },
    {
      "validator": "0x544fec0d957816c880f1ac4c4ca239feede0ac70",
      "staker": "0x307839373161e280a633656338",
      "active": true,
      "jailed": false,
      "should_vote": 710,
      "voted": 710,
      "pubkey": "0xd130582f29b5651854282c80b76c99cf33141fb63f10a69c4a40fa462b94d645",
      "pubkey_type": 2,
      "rate": "0",
      "power": "82649831157500582388751821",
      "unbound_amount": "2821110534212999999999999",
      "punish_rate": "999999880000006590",
      "begin_block": 4636000,
      "unjail_time": 0,
      "memo": {
        "desc": "",
        "logo": "",
        "name": "51%crypto",
        "website": ""
      }
    },
    {
      "validator": "0x61ed9d4018b10e9b007d200725cca0087544268f",
      "staker": "0x307862323362e280a630383539",
      "active": true,
      "jailed": false,
      "should_vote": 710,
      "voted": 710,
      "pubkey": "0x1cf1c137d0e58a9e95310d35ded8f28bd8407d453ce820b34249cf1f92765d99",
      "pubkey_type": 2,
      "rate": "10000",
      "power": "70432535614312586587910445",
      "unbound_amount": "0",
      "punish_rate": "999999820000015284",
      "begin_block": 4636000,
      "unjail_time": 0,
      "memo": {
        "desc": "",
        "logo": "",
        "name": "Fimgent",
        "website": ""
      }
    },
    {
      "validator": "0x805b1f87212164fd1db64b8ed63a8f2c42aac647",
      "staker": "0x307833656465e280a630343831",
      "active": true,
      "jailed": false,
      "should_vote": 710,
      "voted": 710,
      "pubkey": "0xb2b977cd1c0dab54eed96590bebbdee45530ef9d962f7a257d95220ca70563f3",
      "pubkey_type": 2,
      "rate": "0",
      "power": "54482820810059682678324915",
      "unbound_amount": "0",
      "punish_rate": "997401741319268055",
      "begin_block": 4636000,
      "unjail_time": 1698798611,
      "memo": {
        "desc": "We are Jungle Farmer from India, Your trusted delegation partner, come and stake with us.",
        "logo": "https://drive.google.com/file/d/1otsEJ0EKWfRff5yjxYycijokRXhsTwAk/view?usp=sharing",
        "name": "Jungle Farmer",
        "website": ""
      }
    },
    {
      "validator": "0x5c97ee9b91d90b332813078957e3a96b304791b4",
      "staker": "0x307836353630e280a638613832",
      "active": true,
      "jailed": false,
      "should_vote": 710,
      "voted": 710,
      "pubkey": "0x7a5af3a10dda2a41fed36dd76032b6540bb35019078dc0e16f3adf999397e0ce",
      "pubkey_type": 2,
      "rate": "10000",
      "power": "48717022993311051119131086",
      "unbound_amount": "0",
      "punish_rate": "999999880000006590",
      "begin_block": 4636000,
      "unjail_time": 0,
      "memo": {
        "desc": "Managed by a group of talents in blockchain industry, Nodest01 strives to help community engagement through providing education and the best service possible by focusing on relationships over transactions.",
        "logo": "https://drive.google.com/drive/folders/16XJz2179RIkEaoqQnDSJXvBbmK2d8D5I",
        "name": "Nodest01",
        "website": "https://twitter.com/nodest01"
      }
    }
  ]
}
```

<h3 id="1.2">1.2 获取validator最近20笔质押变化</h3>

* `GET /api/diff/latest`
* 参数 

| 参数        | 类型     | 必传 | 说明          |
|-----------|--------|----|-------------|
| validator | string | Y  | validator地址 |

* Request: `http://localhost/api/diff/latest?validator=0xc8d2d4ff0b882243f82c1fb20574c81e4c866e72` 
* Response:  
  * 按高度降序排列
  * 返回值中`amount`，非零正数表示delegate的数量，非零负数表示undelegate的数量
  * 如果`amount`是0，则用`op`区分，`op`为零表示delegate，非零表示undelegate
```json
[
  {
    "block_num": 4636000,
    "total": "9087659109077000000000000",
    "delegator": "0x52f99e02d012ead8fd060dcf1c2ef43e5c327b2d",
    "amount": "27943311200000000000000",
    "op": 0
  },
  {
    "block_num": 4636000,
    "total": "9059715797877000000000000",
    "delegator": "0x1e6cd3ae04429e750d5757c82591d21658b5e7f8",
    "amount": "9979764246000000000000",
    "op": 0
  },
  {
    "block_num": 4636000,
    "total": "9049736033631000000000000",
    "delegator": "0x653318801c8c5c36048895211e0863b2f6315b36",
    "amount": "8795121189790000000000000",
    "op": 0
  },
  {
    "block_num": 4636000,
    "total": "254614843841000000000000",
    "delegator": "0x662d6a5b3aadf0c4551750672215d4c2d658420a",
    "amount": "15000000000000000000000",
    "op": 0
  },
  {
    "block_num": 4636000,
    "total": "239614843841000000000000",
    "delegator": "0x001bff14cd00420680e42a36a8493c4363cb97f1",
    "amount": "190093498478000000000000",
    "op": 0
  },
  {
    "block_num": 4636000,
    "total": "49521345363000000000000",
    "delegator": "0x4476cefb5d2f8ea046e20b8443591348905c79a5",
    "amount": "0",
    "op": 0
  },
  {
    "block_num": 4636000,
    "total": "49521345363000000000000",
    "delegator": "0xe2bb4b93ad94d90fea5f06a327563342c6de967b",
    "amount": "0",
    "op": 0
  },
  {
    "block_num": 4636000,
    "total": "49521345363000000000000",
    "delegator": "0xe4e2ce2f69f3ab481f34a2ab66e1dd9c9e55346e",
    "amount": "0",
    "op": 0
  },
  {
    "block_num": 4636000,
    "total": "49521345363000000000000",
    "delegator": "0x19da6ead98c462be78f8d458ba45c98c29e8adb3",
    "amount": "13380219379000000000000",
    "op": 0
  },
  {
    "block_num": 4636000,
    "total": "36141125984000000000000",
    "delegator": "0xc8d2d4ff0b882243f82c1fb20574c81e4c866e72",
    "amount": "31074614192000000000000",
    "op": 0
  },
  {
    "block_num": 4636000,
    "total": "5066511792000000000000",
    "delegator": "0x3e6689e50a6d1610321b2c01f2c7d7742c586b24",
    "amount": "6766508222000000000000",
    "op": 0
  },
  {
    "block_num": 4636000,
    "total": "-1699996430000000000000",
    "delegator": "0xe4e2ce2f69f3ab481f34a2ab66e1dd9c9e55346e",
    "amount": "-1699996430000000000000",
    "op": 1
  }
]
```
<h3 id="1.3">1.3 获取validator的delegate记录</h3>

* `GET /api/records/delegate`  
* 参数 

| 参数        | 类型     | 必传 | 说明                       |
|-----------|--------|----|--------------------------|
| validator | string | N  | validator地址，不传则返回所有的质押记录 |
| page      | number | N  | 页码，默认1                   |
| page_size | number | N  | 页大小，默认10                 |

* Request: `http://localhost/api/records/delegate?validator=0xc8d2d4ff0b882243f82c1fb20574c81e4c866e72&page=1&page_size=5` 
* Response: 
  * 按`timestamp`降序排列
```json
{
	"total": 11,
	"page": 1,
	"page_size": 5,
	"data": [{
			"block_hash": "0xbfb8f84a77c3ee02f7cf40d2dee62ac9c713ba77cb4ce90f87b0716ca09e8dd5",
			"validator": "0xd518c4f95a3f39ed853a2614566897c4ad5a008f",
			"delegator": "0x876ffa3e317d609438d87ecb55eabb71217f9206",
			"amount": "33000000000000000000",
			"timestamp": 1695724636
		},
		{
			"block_hash": "0x038da9d3b8c6e080f24aef8d61dbd03ddcc6903da3dd8c92f3d655243716503d",
			"validator": "0x69e2b6c4c1122172e69af48e0aec36b7f7c8005a",
			"delegator": "0xccb4e8b208a468f6323312a962c07c2f75ef8eb7",
			"amount": "1862000000000000000000",
			"timestamp": 1695724474
		},
		{
			"block_hash": "0x16b2eb7fc1972feeb09b551119a4506de50a5f796a81820908184b681f5f2664",
			"validator": "0xb4989bbb38287c2af6df0155b55e4073da6c4ba8",
			"delegator": "0x876ffa3e317d609438d87ecb55eabb71217f9206",
			"amount": "300000000000000000000",
			"timestamp": 1695724400
		},
		{
			"block_hash": "0x1405a29669ef9530295ada73881c6a41a66806cbba1b086f6d022b1cc42f4cf6",
			"validator": "0x431500ee574ce0c22bfad987fb4054185d5e8af2",
			"delegator": "0x6348f62079d48e3b6fd35d98aeb55d3eadfa56a9",
			"amount": "547624830951000000000000",
			"timestamp": 1695722301
		},
		{
			"block_hash": "0x1405a29669ef9530295ada73881c6a41a66806cbba1b086f6d022b1cc42f4cf6",
			"validator": "0x68299681f8cd2a772c2dd3d2d2d9c498d46f82ed",
			"delegator": "0xc813c256f3f89b190e0ab86a5fe87845f9cba84b",
			"amount": "0",
			"timestamp": 1695722301
		}
	]
}
```

<h3 id="1.4">1.4 获取validator的undelegate记录</h3>

* `GET /api/records/undelegate`  
* 参数  

| 参数        | 类型     | 必传 | 说明                        |
|-----------|--------|----|---------------------------|
| validator | string | N  | validator地址，不传则返回所有的解质押记录 |
| page      | number | N  | 页码，默认1                    |
| page_size | number | N  | 页大小，默认10                  |

* Request: `http://localhost/api/records/undelegate?validator=0x6e20c920f1bdb817f0e19cd05dae01c6affa5228&page=1&page_size=10` 
* Response: 
  * 按`timestamp`降序排列
```json
{
	"total": 2,
	"page": 1,
	"page_size": 10,
	"data": [{
			"block_hash": "0x1405a29669ef9530295ada73881c6a41a66806cbba1b086f6d022b1cc42f4cf6",
			"validator": "0x6e20c920f1bdb817f0e19cd05dae01c6affa5228",
			"delegator": "0xe9c876ed4622720d994bf141d90b3063bc373af8",
			"amount": "28184996145912000000000000",
			"timestamp": 1695722301
		},
		{
			"block_hash": "0x1405a29669ef9530295ada73881c6a41a66806cbba1b086f6d022b1cc42f4cf6",
			"validator": "0x6e20c920f1bdb817f0e19cd05dae01c6affa5228",
			"delegator": "0xa78edfdea57fd25c7945eb5badb38c4a163864a0",
			"amount": "99469830000000000000000",
			"timestamp": 1695722301
		}
	]
}
```

<h3 id="1.5">1.5 获取validator最近10日vote变化</h3>


* `GET /api/diff/vote`
* 参数

| 参数        | 类型     | 必传 | 说明          |
|-----------|--------|----|-------------|
| validator | string | Y  | validator地址 |
| page      | number | N  | 页码，默认1      |
| page_size | number | N  | 页大小，默认10    |

* Request: `http://localhost/api/diff/vote?validator=0x000e33ab7471186f3b1de9fc08bb9c480f453590&page=1&page_size=5`
* Response:
```json
{
  "total": 65,
  "page": 1,
  "page_size": 5,
  "data": [
    {
      "block_num": 5475318,
      "should_vote": 1135,
      "voted": 1135
    },
    {
      "block_num": 5475317,
      "should_vote": 1134,
      "voted": 1134
    },
    {
      "block_num": 5475316,
      "should_vote": 1134,
      "voted": 1134
    },
    {
      "block_num": 5475314,
      "should_vote": 1131,
      "voted": 1131
    },
    {
      "block_num": 5475313,
      "should_vote": 1130,
      "voted": 1130
    }
  ]
}
```

<h3 id="1.6">1.6 获取质押到validator的delegator集合</h3>

* `GET /api/validator/delegators`
* 参数

| 参数        | 类型     | 必传 | 说明          |
|-----------|--------|----|-------------|
| validator | string | Y  | validator地址 |
| page      | number | N  | 页码，默认1      |
| page_size | number | N  | 页大小，默认10    |

* Request: `http://localhost/api/validator/delegators?validator=0x431500ee574ce0c22bfad987fb4054185d5e8af2&page=1&page_size=10`
* Response: 
  * 按`amount`（质押总量）降序排列
```json
{
  "total": 312,
  "page": 1,
  "page_size": 10,
  "data": [{
    "delegator": "0x450151c996f45125909fd1d29d1209a670854033",
    "amount": "7269283505162000000000000",
    "rank": 1
  }, {
    "delegator": "0xb9c02134ea5e653f86ec05c45ebadd9838f123e1",
    "amount": "6914050464581000000000000",
    "rank": 2
  }, {
    "delegator": "0x52306cf77d30cafd00b47012ccf478b6843838ec",
    "amount": "4000000000000000000000000",
    "rank": 3
  }, {
    "delegator": "0x22fe1fcaf9aa38addd1449579c0b26f536086b18",
    "amount": "2599153895486000000000000",
    "rank": 4
  }, {
    "delegator": "0x7ccaa051d86151f7e9ade047f10a832f3a8e8072",
    "amount": "2204381223589000000000000",
    "rank": 5
  }, {
    "delegator": "0x7db6c2e88ad6a888672d9277618f6bcd716b5ad0",
    "amount": "2128833358001000000000000",
    "rank": 6
  }, {
    "delegator": "0x1e579ae7c8c3bbc2786f6e955e92795b79697d1d",
    "amount": "2102925374625000000000000",
    "rank": 7
  }, {
    "delegator": "0x3b2c16adcb301c6f59d79ac3ef849dcf52ff8e59",
    "amount": "2072365190000000000000000",
    "rank": 8
  }, {
    "delegator": "0x8600679f104dd2aa6600afbd1fef852f2e760112",
    "amount": "1724908374908000000000000",
    "rank": 9
  }, {
    "delegator": "0xb8e8ab3d9567d45d7bc59c0123de865ba870e935",
    "amount": "1500032000000000000000000",
    "rank": 10
  }]
}
```

<h3 id="1.7">1.7 获取delegator所质押的validator集合</h3>

* `GET /api/delegator/validators`
* 参数

| 参数        | 类型     | 必传 | 说明          |
|-----------|--------|----|-------------|
| delegator | string | Y  | delegator地址 |
| page      | number | N  | 页码，默认1      |
| page_size | number | N  | 页大小，默认10    |

* Request: `http://localhost/api/delegator/validators?delegator=0xc813c256f3f89b190e0ab86a5fe87845f9cba84b&page=1&page_size=20`
* Response:
```json
{
  "total": 4,
  "page": 1,
  "page_size": 10,
  "data": [
    "0x431500ee574ce0c22bfad987fb4054185d5e8af2",
    "0x68299681f8cd2a772c2dd3d2d2d9c498d46f82ed",
    "0x971a6ec907d404fb464c123e09faa1d10de13ec8",
    "0xd87c24a44cbe0d41f204a6730ede8d163ed4535a"
  ]
}
```

<h3 id="1.8">1.8 获取地址（delegator）的delegate记录</h3>

* `GET /api/records/delegate/delegator`
* 参数

| 参数        | 类型     | 必传 | 说明                     |
|-----------|--------|----|------------------------|
| delegator | string | N  | delegator地址，不传则返回所有的记录 |
| page      | number | N  | 页码，默认1                 |
| page_size | number | N  | 页大小，默认10               |

* Request:
  * 查询所有delegator的记录：`http://localhost:3000/api/records/delegate/delegator?page=1&page_size=5`
  * 查询特定delegator的记录: `http://localhost/api/records/delegate/delegator?delegator=0x876ffa3e317d609438d87ecb55eabb71217f9206&page=1&page_size=5`
* Response: 
  * 按timestamp降序排列
```json
{
  "total": 5113,
  "page": 1,
  "page_size": 5,
  "data": [{
    "block_hash": "0xbfb8f84a77c3ee02f7cf40d2dee62ac9c713ba77cb4ce90f87b0716ca09e8dd5",
    "validator": "0xd518c4f95a3f39ed853a2614566897c4ad5a008f",
    "delegator": "0x876ffa3e317d609438d87ecb55eabb71217f9206",
    "amount": "33000000000000000000",
    "timestamp": 1695724636
  },
    {
      "block_hash": "0x038da9d3b8c6e080f24aef8d61dbd03ddcc6903da3dd8c92f3d655243716503d",
      "validator": "0x69e2b6c4c1122172e69af48e0aec36b7f7c8005a",
      "delegator": "0xccb4e8b208a468f6323312a962c07c2f75ef8eb7",
      "amount": "1862000000000000000000",
      "timestamp": 1695724474
    },
    {
      "block_hash": "0x16b2eb7fc1972feeb09b551119a4506de50a5f796a81820908184b681f5f2664",
      "validator": "0xb4989bbb38287c2af6df0155b55e4073da6c4ba8",
      "delegator": "0x876ffa3e317d609438d87ecb55eabb71217f9206",
      "amount": "300000000000000000000",
      "timestamp": 1695724400
    },
    {
      "block_hash": "0x1405a29669ef9530295ada73881c6a41a66806cbba1b086f6d022b1cc42f4cf6",
      "validator": "0xbef69a5f5e5ecd3dd9d7f829c6b693f760179b63",
      "delegator": "0x663da7e09c849cad59670a0844a4043dff2b89c4",
      "amount": "8779971026000000000000",
      "timestamp": 1695722301
    },
    {
      "block_hash": "0x1405a29669ef9530295ada73881c6a41a66806cbba1b086f6d022b1cc42f4cf6",
      "validator": "0xc8d2d4ff0b882243f82c1fb20574c81e4c866e72",
      "delegator": "0x52f99e02d012ead8fd060dcf1c2ef43e5c327b2d",
      "amount": "27943311200000000000000",
      "timestamp": 1695722301
    }
  ]
}
```

<h3 id="1.9">1.9 获取地址（delegator）的undelegate记录</h3>

* `GET /api/records/undelegate/delegator`
* 参数

| 参数        | 类型     | 必传 | 说明                     |
|-----------|--------|----|------------------------|
| delegator | string | N  | delegator地址，不传则返回所有的记录 |
| page      | number | N  | 页码，默认1                 |
| page_size | number | N  | 页大小，默认10               |


* Request:
  * 查询所有delegator的记录：`http://localhost/api/records/undelegate/delegator?page=1&page_size=5`
  * 查询特定delegator的记录: `http://localhost/api/records/undelegate/delegator?delegator=0x2205f3969aab00fa34bcb6c0b636e8a1c5624bedpage=1&page_size=5`
* Response:
  * 按timestamp降序排列
```json
{
	"total": 122,
	"page": 1,
	"page_size": 5,
	"data": [{
		"block_hash": "0xf928683f9076a94c9bac6c1a93ccf4aa20e489d1c6fbef2d23a90bec675f1a1d",
		"validator": "0xf0eab3a51d7f32fcd8a6c6f473a8d70d3bfff2d4",
		"delegator": "0x2205f3969aab00fa34bcb6c0b636e8a1c5624bed",
		"amount": "9999980000000000000000000",
		"timestamp": 1695728413
	}, {
		"block_hash": "0xec27de37894cd567e029abbc5ebc23619c33020a77de3cd78e0df6b62d2c35c2",
		"validator": "0x847248d9dcd36acc070f5cdc9541a84f8b5246cd",
		"delegator": "0x2205f3969aab00fa34bcb6c0b636e8a1c5624bed",
		"amount": "103236566237000000000000",
		"timestamp": 1695727870
	}, {
		"block_hash": "0x1f2d343e0b7294b95eed4315685cb73f2820c8601f7bae7573f9fa7cdd5356eb",
		"validator": "0xf0eab3a51d7f32fcd8a6c6f473a8d70d3bfff2d4",
		"delegator": "0x2205f3969aab00fa34bcb6c0b636e8a1c5624bed",
		"amount": "9999980000000000000000000",
		"timestamp": 1695727122
	}, {
		"block_hash": "0xdf69e99b12ffea1dc08ceaad9f4e0be7258ac44b87f09e142b86ac18800ff424",
		"validator": "0x0c6fe866cb0fc52d9214276d2bd9327610764ab1",
		"delegator": "0x2205f3969aab00fa34bcb6c0b636e8a1c5624bed",
		"amount": "386944000000000000",
		"timestamp": 1695726647
	}, {
		"block_hash": "0x055a0813301a43f8cbbe4be77cfdb206cc3ea8806806b9c871ec777b3188c64c",
		"validator": "0x847248d9dcd36acc070f5cdc9541a84f8b5246cd",
		"delegator": "0x9a2cf38e324e6ad815e086161ff8fdc8cdd5e6cf",
		"amount": "19764620608000000000000",
		"timestamp": 1695726534
	}]
}
```

<h3 id="2.1">2.1 获取bound数量</h3>

* `GET /api/bound` 
* 参数 

| 参数        | 类型     | 必传 | 说明          |
|-----------|--------|----|-------------|
| validator | string | Y  | validator地址 |
| delegator | string | Y  | delegator地址 |

* Request: `http://localhost/api/bound?validator=0x09ef1db6b67d1cbf7eba6bd9b204611848993df7&delegator=0x2d15d52cc138ffb322b732239cd3630735abac88` 
* Response: 
```json
{
  "bound_amount": "110000001800000063120",
  "unbound_amount": "0"
}
```


<h3 id="2.2">2.2 获取reward数量</h3>

* `GET /api/reward` 
* 参数 

| 参数      | 类型     | 必传 | 说明          |
|---------|--------|----|-------------|
| address | string | Y  | delegator地址 |

* Request: `http://localhost/api/reward?address=0x2d15d52cc138ffb322b732239cd3630735abac88` 
* Response: 
```json
{
  "reward": "16742332457649244907"
}
```

<h3 id="2.3">2.3 获取debt数量</h3>

* `GET /api/debt` 
* 参数 

| 参数        | 类型     | 必传 | 说明          |
|-----------|--------|----|-------------|
| validator | string | Y  | validator地址 |
| delegator | string | Y  | delegator地址 |

* Request: `http://localhost/api/debt?validator=0xd518c4f95a3f39ed853a2614566897c4ad5a008f&delegator=0x2d15d52cc138ffb322b732239cd3630735abac88` 
* Response: 
```json
{
  "debt": "96558069283467635"
}
```

<h3 id="2.4">2.4  获取validator数据</h3>

* `GET /api/vdata`
* 参数

| 参数      | 类型     | 必传 | 说明          |
|---------|--------|----|-------------|
| address | string | Y  | validator地址 |

* Request: `http://localhost/api/vdata?address=0xd518c4f95a3f39ed853a2614566897c4ad5a008f`
* Response:
```json
{
  "public_key": "0x7d8cd5a5c3560717c356c25dfd5ac4dba7041b5fe97f0c94ff5e095876b29e93",
  "public_key_type": 2,
  "rate": "50000",
  "staker": "0x950b02daba544942456fc3158b49d2f3c2d83e26",
  "power": "173888919852423290891303799",
  "total_unbound_amount": "65869059538217509715792833",
  "begin_block": 999945591479922200
}
```

<h3 id="2.5">2.5  获取validator状态</h3>

* `GET /api/vstatus`
* 参数

| 参数      | 类型     | 必传 | 说明          |
|---------|--------|----|-------------|
| address | string | Y  | validator地址 |


* Request: `http://localhost/api/vstatus?address=0xd518c4f95a3f39ed853a2614566897c4ad5a008f`
* Response:
```json
{
  "heap_index_off1": "54",
  "is_active": true,
  "jailed": false,
  "unjail_datetime": 1697279406,
  "should_vote": 1445,
  "voted": 1445
}
```

<h3 id="3.1">3.1 统计delegate,undelegate,claim总量</h3>

* `GET /api/sum` 
* 参数 

| 参数      | 类型     | 必传 | 说明 |
|---------|--------|----|----|
| address | string | Y  | 地址 |

* Request: `http://localhost/api/sum?address=0xeb2b96369e83e1466bb56f2bf9d97cbda130e741` 
* Response: 
```json
{
  "delegate": "32254951206000000000000",
  "undelegate": "0",
  "claim": "0"
}
```
