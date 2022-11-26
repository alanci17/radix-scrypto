-------------------------------------------------------------------------------------------
# Index  
-------------------------------------------------------------------------------------------	
> [Part_1](#part_1) . Test "stock/restock/unstock" candy methods.
>	+  [1.1](#part_1_1) . "stock_candy" method
>	+  [1.2](#part_1_2) . "restock_candy" method
>	+  [1.3](#part_1_3) . "stock_position" method
>	+  [1.4](#part_1_4) . "unstock_candy" method
>
> [Part_2](#part_2) . Test swap methods.
>	+  [2.1](#part_2_1) . "buy_exact_candy_sell_xrd" method
>	+  [2.2](#part_2_2) . "buy_candy_sell_exact_xrd" method
>	+  [2.3](#part_2_3) . "buy_exact_xrd_sell_candy" method
>	+  [2.4](#part_2_4) . "buy_xrd_sell_exact_candy" method
>	+  [2.5](#part_2_5) . "buy_candy_sell_exact_candy" method
>	+  [2.6](#part_2_6) . "buy_candy_sell_exact_candy" method
>		
> [Part_3](#part_3) . Test "flashswap" method.
> 	+  [3.1](#part_3_1) . Borrow XRD & reimburse XRD
> 	+  [3.2](#part_3_2) . Borrow Candy & reimburse a different Candy
> 	+  [3.3](#part_3_3) . Borrow Candy & reimburse XRD
> 	+  [3.4](#part_3_4) . Example of reverted transaction due to unprofitable "flashswap" method call
-------------------------------------------------------------------------------------------
# Wording  
-------------------------------------------------------------------------------------------

>function  [String]	```name_of_the_function``` 
> 
>method    [String]	```name_of_the_method```  
>
>arguments [Bucket] 	```[Decimal(Bucket_amount)]),[ResourceAddress(Bucket_resource_address)])```
>
>arguments [resource] 	```[ResourceAddress(resource_address)])```
>
>arguments [component]	```[ComponentAddress(component_address)])```
>
>arguments [value] 	```[Decimal(amount)])```

#
### Part_1 
# Let's test "stock/restock/unstock" candy methods.
-------------------------------------------------------------------------------------------
Simulator reset & new Default-account generation
-------------------------------------------------------------------------------------------

>resim reset
>
>resim new-account
```
├─ Account address: 0293c502780e23621475989d707cd8128e4506362e5fed6ac0c00a = $Default-account

└─ Public key: 005feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9 = $Public-key
```
-------------------------------------------------------------------------------------------
Publish CandyDex Blueprint & Component instantiate 
-------------------------------------------------------------------------------------------

>resim publish .
```
└─ Package: 013fa22e238526e9c82376d2b4679a845364243bf970e5f783d13f = $Package
```

>function ```new```  arguments ```1``` 

> cd transaction_manifest
> 
> modify data on ```candydex_instantiate```
> 
> ```candydex_instantiate.sh```

```
├─ ResourceDef: 03eb23d0867f32265935d93970aded9033cc868d31795f27d8cb62 = $MinterBadge

├─ ResourceDef: 0347dfe3a58e8a630305f2f3df82949cd70ce49e2cde097b259f8d = $OwnerBadge

└─ Component: 02ac00a15a87df7c43b55e49d5d229bc770136c108586a9d7b38b5   = $CandyDex
```

-------------------------------------------------------------------------------------------
Let's check out CandyDex Component.
-------------------------------------------------------------------------------------------

>resim show $CandyDex
```
├─ { amount: 0, resource_def: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" } = $XRD

└─ { amount: 1, resource_def: 03eb23d0867f32265935d93970aded9033cc868d31795f27d8cb62, name: " MinterBadge " }
```
-------------------------------------------------------------------------------------------
Let's create some candy tokens.
-------------------------------------------------------------------------------------------

>resim new-token-fixed --name "THETAGUM" 100000 --symbol "THG"
```
└─ ResourceDef: 034c9bafe1e39e4a695e617202eddffb954b6b19c3c21fcd9a7677 = $THG
```

---
>resim new-token-fixed --name "OMICRONGUM" 100000 --symbol "OMG"
```
└─ ResourceDef: 033f8829bea3d849592fb5dfba1f94d4a95c5683d43f09e78a56d7 = $OMG
```

-------------------------------------------------------------------------------------------
Let's check out our Default-account.
-------------------------------------------------------------------------------------------

>resim show $Default-account
```
├─ { amount: 1000000, resource_def: $XRD, name: "Radix", symbol: "XRD" }   

├─ { amount: 100000, resource_def: $THG, name: "THETAGUM", symbol: "THG" }

└─ { amount: 100000, resource_def: $OMG, name: "OMICRONGUM", symbol: "OMG" }
```
-------------------------------------------------------------------------------------------
#
### part_1_1
Let's stock candies, inspect resturned resources and Default-account balances.
-------------------------------------------------------------------------------------------

>method ```stock_candy```  arguments ```10000,$THG 2``` 

> cd transaction_manifest
> 
> modify data on ```stock_candy```
> 
> ```stock_candy.sh```

```
└─ [←[32mINFO ←[0m] ←[32m Added 10000 THETAGUM candy, THG symbol @2XRD price

├─ ResourceDef: 0308c9a9f364730bfe280db8543feb06540aa240a54d4274cd3d73  = $mTHETAGUM

└─ ResourceDef: 03411b8e24f4acfd9b8f35d6089fa892521ddaf86d7a07aa75dbd5  = $mBadgeTHG_0
```

---

>method ```stock_candy```  arguments ```10000,$OMG 2```

> cd transaction_manifest
> 
> modify data on ```stock_candy```
> 
> ```stock_candy.sh```
 
```
└─ [←[32mINFO ←[0m] ←[32m Added 10000 OMICRONGUM candy, OMG symbol @2XRD price

├─ ResourceDef: 03f7c9f4e360270a74b3d90207272eda123ae05df1f35aab17d20e  = $mOMICRONGUM

└─ ResourceDef: 03634189be8ce5e3a50bcc95ff7291669b8f7666e86008ab827692  = $mBadgeOMG_0
```

---
>resim show $Default-account
```
├─ { amount: 1, resource_def: $mBadgeTHG_0, symbol: " mBadgeTHG" }

├─ { amount: 10000, resource_def: $mTHETAGUM, name: " mTHETAGUM", symbol: " mTHG" }

├─ { amount: 90000, resource_def: $THG, name: "THETAGUM", symbol: "THG" }

├─ { amount: 1, resource_def: $mBadgeOMG_0, symbol: " mBadgeOMG" }

├─ { amount: 10000, resource_def: $mOMICRONGUM, name: " mOMICRONGUM", symbol: " mOMG" }

└─ { amount: 90000, resource_def: $OMG, name: "OMICRONGUM", symbol: "OMG" }
```

-------------------------------------------------------------------------------------------
Let's swap some candies to gain some accrued fee profit.  
-------------------------------------------------------------------------------------------

>method ```stock_candy```  arguments ```500 $THG 2000,$OMG```

> cd transaction_manifest
> 
> modify data on ```buy_exact_candy_sell_candy```
> 
> ```buy_exact_candy_sell_candy.sh```

---
>method ```stock_candy```  arguments ```500 $OMG 2000,$THG```

> cd transaction_manifest
> 
> modify data on ```buy_exact_candy_sell_candy```
> 
> ```buy_exact_candy_sell_candy.sh```

---
>method ```stock_candy```  arguments ```500 $THG 2000,$OMG```

> cd transaction_manifest
> 
> modify data on ```buy_exact_candy_sell_candy```
> 
> ```buy_exact_candy_sell_candy.sh```

---
>method ```stock_candy```  arguments ```500 $OMG 2000,$THG```

> cd transaction_manifest
> 
> modify data on ```buy_exact_candy_sell_candy```
> 
> ```buy_exact_candy_sell_candy.sh```

-------------------------------------------------------------------------------------------
Let's try to stock same candies again using "restock_candy" method. 
-------------------------------------------------------------------------------------------

>method ```restock_candy```  arguments ```10000,$THG```

> cd transaction_manifest
> 
> modify data on ```restock_candy```
> 
> ```restock_candy.sh```

```
├─ [←[32mINFO ←[0m] ←[32m Added 10000 THETAGUM candy, THG symbol @2XRD price

└─ [←[32mINFO ←[0m] ←[32m entry_fee 10.10101010101010101 

└─ ResourceDef: 0395c6abce3ea0ea35e88cc157ef1acf483fb9f3043ee038991734 = $mBadgeTHG_1
```
---
>method ```restock_candy```  arguments ```10000,$OMG```

> cd transaction_manifest
> 
> modify data on ```restock_candy```
> 
> ```restock_candy.sh```

```
├─ [←[32mINFO ←[0m] ←[32m Added 10000 OMICRONGUM candy, OMG symbol @2XRD price

└─ [←[32mINFO ←[0m] ←[32m entry_fee 10.10101010101010101 

└─ ResourceDef: 032a51d207b03508eebfa7a758901e1129cfc6178cb1ca3f009c76 = $mBadgeOMG_1
```

-------------------------------------------------------------------------------------------
Let's swap some candies to gain some accrued fee profit.  
-------------------------------------------------------------------------------------------


>method ```buy_exact_candy_sell_candy```  arguments ```500 $THG 2000,$OMG```

> cd transaction_manifest
> 
> modify data on ```buy_exact_candy_sell_candy```
> 
> ```buy_exact_candy_sell_candy.sh```

---

>method ```buy_exact_candy_sell_candy```  arguments ```500 $OMG 2000,$THG```

> cd transaction_manifest
> 
> modify data on ```buy_exact_candy_sell_candy```
> 
> ```buy_exact_candy_sell_candy.sh```

---

>method ```buy_exact_candy_sell_candy```  arguments ```500 $THG 2000,$OMG```

> cd transaction_manifest
> 
> modify data on ```buy_exact_candy_sell_candy```
> 
> ```buy_exact_candy_sell_candy.sh```

---

>method ```buy_exact_candy_sell_candy```  arguments ```500 $OMG 2000,$THG```

> cd transaction_manifest
> 
> modify data on ```buy_exact_candy_sell_candy```
> 
> ```buy_exact_candy_sell_candy.sh```

-------------------------------------------------------------------------------------------
#
### part_1_2
Let's try to stock same candies again using "restock_candy" method. 
-------------------------------------------------------------------------------------------

>method ```restock_candy```  arguments ```10000,$THG```

> cd transaction_manifest
> 
> modify data on ```restock_candy```
> 
> ```restock_candy.sh```

```
├─ [←[32mINFO ←[0m] ←[32m Added 10000 THETAGUM candy, THG symbol @2XRD price

└─ [←[32mINFO ←[0m] ←[32m entry_fee 20.20202020202020202 

└─ ResourceDef: 036bdf5a7892cb113b83621a2718bb69047490e8a8f6819b28a07d = $mBadgeTHG_2
```

---
>method ```restock_candy```  arguments ```10000,$OMG```

> cd transaction_manifest
> 
> modify data on ```restock_candy```
> 
> ```restock_candy.sh```

```
├─ [←[32mINFO ←[0m] ←[32m Added 10000 OMICRONGUM candy, OMG symbol @2XRD price

└─ [←[32mINFO ←[0m] ←[32m entry_fee 20.20202020202020202 

└─ ResourceDef: 03a021cd3cde156353af7ebb97f4d81c09aca3d5ded91eea38e4a6 = $mBadgeOMG_2
```

-------------------------------------------------------------------------------------------
Let's swap some candies to gain some accrued fee profit.  
-------------------------------------------------------------------------------------------

>method ```buy_exact_candy_sell_candy```  arguments ```500 $THG 2000,$OMG```

> cd transaction_manifest
> 
> modify data on ```buy_exact_candy_sell_candy```
> 
> ```buy_exact_candy_sell_candy.sh```

---

>method ```buy_exact_candy_sell_candy```  arguments ```500 $OMG 2000,$THG```

> cd transaction_manifest
> 
> modify data on ```buy_exact_candy_sell_candy```
> 
> ```buy_exact_candy_sell_candy.sh```

---

>method ```buy_exact_candy_sell_candy```  arguments ```500 $THG 2000,$OMG```

> cd transaction_manifest
> 
> modify data on ```buy_exact_candy_sell_candy```
> 
> ```buy_exact_candy_sell_candy.sh```

---

>method ```buy_exact_candy_sell_candy```  arguments ```500 $OMG 2000,$THG```

> cd transaction_manifest
> 
> modify data on ```buy_exact_candy_sell_candy```
> 
> ```buy_exact_candy_sell_candy.sh```

-------------------------------------------------------------------------------------------
Let's check out Default-account balances.
-------------------------------------------------------------------------------------------

>resim show $Default-account
```
├─ { amount: 1000000, resource_def: $XRD, name: "Radix", symbol: "XRD" }

├─ { amount: 1, resource_def: $OwnerBadge, name: " OwnerBadge " }

├─ { amount: 1, resource_def: $mBadgeTHG_0, symbol: " mBadgeTHG" }

├─ { amount: 1, resource_def: $mBadgeTHG_2, symbol: " mBadgeTHG" }

├─ { amount: 1, resource_def: $mBadgeTHG_1, symbol: " mBadgeTHG" }

├─ { amount: 1, resource_def: $mBadgeOMG_0, symbol: " mBadgeOMG" }

├─ { amount: 1, resource_def: $mBadgeOMG_1, symbol: " mBadgeOMG" }

├─ { amount: 1, resource_def: $mBadgeOMG_2, symbol: " mBadgeOMG" }

├─ { amount: 70059.988441314613411367, resource_def: $OMG, name: "OMICRONGUM", symbol: "OMG" }

├─ { amount: 70063.332235019889392175, resource_def: $THG, name: "THETAGUM", symbol: "THG" }

├─ { amount: 30000, resource_def: $mOMICRONGUM, name: " mOMICRONGUM", symbol: " mOMG" }

└─ { amount: 30000, resource_def: $mTHETAGUM, name: " mTHETAGUM", symbol: " mTHG" }
```
-------------------------------------------------------------------------------------------
#
### part_1_3
Let's check out our stock positions plus accrued gains with "stock_position" method.
-------------------------------------------------------------------------------------------

>method ```stock_position```  arguments ```1,$mBadgeTHG_0```

> cd transaction_manifest
> 
> modify data on ```stock_position```
> 
> ```stock_position.sh```

```
├─ [←[32mINFO ←[0m] ←[32m accrued_fee 30.30303030303030303 ←[0m

├─ [←[32mINFO ←[0m] ←[32m total_minted 30000 ←[0m

├─ [←[32mINFO ←[0m] ←[32m entry_fee 0 ←[0m

├─ [←[32mINFO ←[0m] ←[32m delta_fee 30.30303030303030303 ←[0m

└─ [←[32mINFO ←[0m] ←[32m candy_out_nbr 10010.10101010101010101 ←[0m
```

---

>method ```stock_position```  arguments ```1,$mBadgeTHG_2```

> cd transaction_manifest
> 
> modify data on ```stock_position```
> 
> ```stock_position.sh```
```
├─ [←[32mINFO ←[0m] ←[32m accrued_fee 30.30303030303030303 ←[0m

├─ [←[32mINFO ←[0m] ←[32m total_minted 30000 ←[0m

├─ [←[32mINFO ←[0m] ←[32m entry_fee 20.20202020202020202 ←[0m

├─ [←[32mINFO ←[0m] ←[32m delta_fee 10.10101010101010101 ←[0m

└─ [←[32mINFO ←[0m] ←[32m candy_out_nbr 10003.367003367003367003 ←[0m
```

---

>method ```stock_position```  arguments ```1,$mBadgeTHG_1```

> cd transaction_manifest
> 
> modify data on ```stock_position```
> 
> ```stock_position.sh```
```
├─ [←[32mINFO ←[0m] ←[32m accrued_fee 30.30303030303030303 ←[0m

├─ [←[32mINFO ←[0m] ←[32m total_minted 30000 ←[0m

├─ [←[32mINFO ←[0m] ←[32m entry_fee 10.10101010101010101 ←[0m

├─ [←[32mINFO ←[0m] ←[32m delta_fee 20.20202020202020202 ←[0m

└─ [←[32mINFO ←[0m] ←[32m candy_out_nbr 10006.734006734006734006 ←[0m
```

---

>method ```stock_position```  arguments ```1,$mBadgeOMG_0```

> cd transaction_manifest
> 
> modify data on ```stock_position```
> 
> ```stock_position.sh```
```
├─ [←[32mINFO ←[0m] ←[32m accrued_fee 30.30303030303030303 ←[0m

├─ [←[32mINFO ←[0m] ←[32m total_minted 30000 ←[0m

├─ [←[32mINFO ←[0m] ←[32m entry_fee 0 ←[0m

├─ [←[32mINFO ←[0m] ←[32m delta_fee 30.30303030303030303 ←[0m

└─ [←[32mINFO ←[0m] ←[32m candy_out_nbr 10010.10101010101010101 ←[0m
```

---

>method ```stock_position```  arguments ```1,$mBadgeOMG_1```

> cd transaction_manifest
> 
> modify data on ```stock_position```
> 
> ```stock_position.sh```
```
├─ [←[32mINFO ←[0m] ←[32m accrued_fee 30.30303030303030303 ←[0m

├─ [←[32mINFO ←[0m] ←[32m total_minted 30000 ←[0m

├─ [←[32mINFO ←[0m] ←[32m entry_fee 10.10101010101010101 ←[0m

├─ [←[32mINFO ←[0m] ←[32m delta_fee 20.20202020202020202 ←[0m

└─ [←[32mINFO ←[0m] ←[32m candy_out_nbr 10006.734006734006734006 ←[0m
```

---

>method ```stock_position```  arguments ```1,$mBadgeOMG_2```

> cd transaction_manifest
> 
> modify data on ```stock_position```
> 
> ```stock_position.sh```
```
├─ [←[32mINFO ←[0m] ←[32m accrued_fee 30.30303030303030303 ←[0m

├─ [←[32mINFO ←[0m] ←[32m total_minted 30000 ←[0m

├─ [←[32mINFO ←[0m] ←[32m entry_fee 20.20202020202020202 ←[0m

├─ [←[32mINFO ←[0m] ←[32m delta_fee 10.10101010101010101 ←[0m

└─ [←[32mINFO ←[0m] ←[32m candy_out_nbr 10003.367003367003367003 ←[0m
```

-------------------------------------------------------------------------------------------
#
### part_1_4
Let's unstock some candies.  
-------------------------------------------------------------------------------------------

>method ```unstock_candy```  arguments ```$THG 10000,$mTHETAGUM 1,$mBadgeTHG_0```

> cd transaction_manifest
> 
> modify data on ```unstock_candy```
> 
> ```unstock_candy.sh```
```
├─ [←[32mINFO ←[0m] ←[32m total_minted 30000 

├─ [←[32mINFO ←[0m] ←[32m accrued_fee 30.30303030303030303 

├─ [←[32mINFO ←[0m] ←[32m entry_fee 0 

├─ [←[32mINFO ←[0m] ←[32m delta_fee 30.30303030303030303 

└─ [←[32mINFO ←[0m] ←[32m candy_out_nbr 10010.10101010101010101
```

---

>method ```unstock_candy```  arguments ```$THG 10000,$mTHETAGUM 1,$mBadgeTHG_1```

> cd transaction_manifest
> 
> modify data on ```unstock_candy```
> 
> ```unstock_candy.sh```
```
├─ [←[32mINFO ←[0m] ←[32m total_minted 30000 

├─ [←[32mINFO ←[0m] ←[32m accrued_fee 30.30303030303030303 

├─ [←[32mINFO ←[0m] ←[32m entry_fee 10.10101010101010101 

├─ [←[32mINFO ←[0m] ←[32m delta_fee 20.20202020202020202 

└─ [←[32mINFO ←[0m] ←[32m candy_out_nbr 10006.734006734006734006
```

---

>method ```unstock_candy```  arguments ```$OMG 10000,$mOMICRONGUM 1,$mBadgeOMG_0```

> cd transaction_manifest
> 
> modify data on ```unstock_candy```
> 
> ```unstock_candy.sh``` 
```
├─ [←[32mINFO ←[0m] ←[32m total_minted 30000 

├─ [←[32mINFO ←[0m] ←[32m accrued_fee 30.30303030303030303 

├─ [←[32mINFO ←[0m] ←[32m entry_fee 0 

├─ [←[32mINFO ←[0m] ←[32m delta_fee 30.30303030303030303 

└─ [←[32mINFO ←[0m] ←[32m candy_out_nbr 10010.10101010101010101
```

---

>method ```unstock_candy```  arguments ```$OMG 10000,$mOMICRONGUM 1,$mBadgeOMG_1```

> cd transaction_manifest
> 
> modify data on ```unstock_candy```
> 
> ```unstock_candy.sh``` 
```
├─ [←[32mINFO ←[0m] ←[32m total_minted 30000 

├─ [←[32mINFO ←[0m] ←[32m accrued_fee 30.30303030303030303 

├─ [←[32mINFO ←[0m] ←[32m entry_fee 10.10101010101010101 

├─ [←[32mINFO ←[0m] ←[32m delta_fee 20.20202020202020202 

└─ [←[32mINFO ←[0m] ←[32m candy_out_nbr 10006.734006734006734006
```

-------------------------------------------------------------------------------------------
Let's issue another candy token, stock it & swap some candies to rebalance CandyDex Component amounts.  
-------------------------------------------------------------------------------------------

>resim new-token-fixed --name "ETAGUM" 100000 --symbol "ETG"
```
└─ ResourceDef: 03a78cfec3dac583cc2394d14452099892a5af4a5201d771d918a2 = $ETG
```

---

>method ```stock_candy```  arguments ```10000,$ETG 2```

> cd transaction_manifest
> 
> modify data on ```stock_candy```
> 
> ```stock_candy.sh``` 
```
└─ [←[32mINFO ←[0m] ←[32m Added 10000 ETAGUM candy, ETG symbol @2XRD price

├─ ResourceDef: 03be62f5e91b4697231a63826c86ec1a597a0e1738e8c5a3ab9ab6

└─ ResourceDef: 034342e2f24c45cc8f34affff1ef96cdeee275ebc19da28d80fe1a
```

---

>method ```buy_exact_candy_sell_candy```  arguments ```500 $ETG 2000,$OMG```

> cd transaction_manifest
> 
> modify data on ```buy_exact_candy_sell_candy```
> 
> ```buy_exact_candy_sell_candy.sh```


---

>method ```buy_exact_candy_sell_candy```  arguments ```500 $ETG 2000,$THG```

> cd transaction_manifest
> 
> modify data on ```buy_exact_candy_sell_candy```
> 
> ```buy_exact_candy_sell_candy.sh``` 


-------------------------------------------------------------------------------------------
Let's unstock last candies.  
-------------------------------------------------------------------------------------------

>method ```unstock_candy```  arguments ```$THG 10000,$mTHETAGUM 1,$mBadgeTHG_2```

> cd transaction_manifest
> 
> modify data on ```unstock_candy```
> 
> ```unstock_candy.sh```
```
├─ [←[32mINFO ←[0m] ←[32m total_minted 30000 

├─ [←[32mINFO ←[0m] ←[32m accrued_fee 30.30303030303030303 

├─ [←[32mINFO ←[0m] ←[32m entry_fee 20.20202020202020202 

├─ [←[32mINFO ←[0m] ←[32m delta_fee 10.10101010101010101 

└─ [←[32mINFO ←[0m] ←[32m candy_out_nbr 10003.367003367003367003
```

---

>method ```unstock_candy```  arguments ```$OMG 5000,$mOMICRONGUM 1,$mBadgeOMG_2```

> cd transaction_manifest
> 
> modify data on ```unstock_candy```
> 
> ```unstock_candy.sh```
```
├─ [←[32mINFO ←[0m] ←[32m total_minted 30000 

├─ [←[32mINFO ←[0m] ←[32m accrued_fee 30.30303030303030303 

├─ [←[32mINFO ←[0m] ←[32m entry_fee 20.20202020202020202 

├─ [←[32mINFO ←[0m] ←[32m delta_fee 10.10101010101010101 

└─ [←[32mINFO ←[0m] ←[32m candy_out_nbr 5001.683501683501683501
```

---

>method ```unstock_candy```  arguments ```$OMG 5000,$mOMICRONGUM 1,$mBadgeOMG_2```

> cd transaction_manifest
> 
> modify data on ```unstock_candy```
> 
> ```unstock_candy.sh```
```
├─ [←[32mINFO ←[0m] ←[32m total_minted 30000 

├─ [←[32mINFO ←[0m] ←[32m accrued_fee 30.30303030303030303 

├─ [←[32mINFO ←[0m] ←[32m entry_fee 20.20202020202020202 

├─ [←[32mINFO ←[0m] ←[32m delta_fee 10.10101010101010101 

└─ [←[32mINFO ←[0m] ←[32m candy_out_nbr 5001.683501683501683501
```

-------------------------------------------------------------------------------------------
Let's check out our Default-account balances.
-------------------------------------------------------------------------------------------

>resim show $Default-account
```
├─ { amount: 0, resource_def: $mBadgeTHG_1, symbol: " mBadgeTHG" }

├─ { amount: 0, resource_def: $mBadgeTHG_0, symbol: " mBadgeTHG" }

├─ { amount: 0, resource_def: $mBadgeTHG_2, symbol: " mBadgeTHG" }

├─ { amount: 0, resource_def: $mTHETAGUM, name: " mTHETAGUM", symbol: " mTHG" }

├─ { amount: 99399.843801283119809953, resource_def: $THG, name: "THETAGUM", symbol: "THG" }

├─ { amount: 0, resource_def: $mBadgeOMG_0, symbol: " mBadgeOMG" }

├─ { amount: 0, resource_def: $mBadgeOMG_1, symbol: " mBadgeOMG" }

├─ { amount: 0, resource_def: $mBadgeOMG_2, symbol: " mBadgeOMG" }

├─ { amount: 0, resource_def: $mOMICRONGUM, name: " mOMICRONGUM", symbol: " mOMG" }

└─ { amount: 99397.576325596498385148, resource_def: $OMG, name: "OMICRONGUM", symbol: "OMG" }
```
[Back Up](#index)
#
### Part_2 
# Let's test swap methods.
-------------------------------------------------------------------------------------------
Simulator reset & new Default-account generation
-------------------------------------------------------------------------------------------


>resim reset
>
>resim new-account
```
├─ Account address: 02ffa01926302c78c0f050f6d08140ec77757ec6cd277f7eecef42 = $Default-account

└─ Public key: 005feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9 = $Public-key
```                    
-------------------------------------------------------------------------------------------
Publish CandyDex Blueprint & Component instantiate 
-------------------------------------------------------------------------------------------

>resim publish .
```
└─ Package: 01ca59a8d6ea4f7efa1765cef702d14e47570c079aedd44992dd09 = $Package
```
---
>resim call-function $Package CandyDex new 1 
```
├─ ResourceDef: 03eb23d0867f32265935d93970aded9033cc868d31795f27d8cb62 = $MinterBadge

├─ ResourceDef: 0399d3f4678fbf0ec6abb57bb17af7ddcc48ce1370e65eb99f8e13 = $OwnerBadge

└─ Component: 02ac00a15a87df7c43b55e49d5d229bc770136c108586a9d7b38b5   = $CandyDex
```

-------------------------------------------------------------------------------------------
Let's check out CandyDex Component.
-------------------------------------------------------------------------------------------

>resim show $CandyDex
```
├─ { amount: 0, resource_def: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" } = $XRD

└─ { amount: 1, resource_def: 03eb23d0867f32265935d93970aded9033cc868d31795f27d8cb62, name: " MinterBadge " }
```
-------------------------------------------------------------------------------------------
Let's create some candy tokens.
-------------------------------------------------------------------------------------------

>resim new-token-fixed --name "ALPHAGUM" 100000 --symbol "ALG"
```
└─ ResourceDef: $ALG
```

---
>resim new-token-fixed --name "BETAGUM" 100000 --symbol "BTG"
```
└─ ResourceDef: $BTG
```

-------------------------------------------------------------------------------------------
Let's check out our Default-account balances.
-------------------------------------------------------------------------------------------

>resim show $Default-account
```
├─ { amount: 1, resource_def: $OwnerBadge, name: " OwnerBadge " }

├─ { amount: 1000000, resource_def: $XRD, name: "Radix", symbol: "XRD" } 

├─ { amount: 100000, resource_def: $BTG, name: "BETAGUM", symbol: "BTG" }

├─ { amount: 100000, resource_def: $ALG, name: "ALPHAGUM", symbol: "ALG" }
```
-------------------------------------------------------------------------------------------
Let's stock candies and check Default-account balances.
-------------------------------------------------------------------------------------------

>method ```stock_candy```  arguments ```50000,$ALG 2```

> cd transaction_manifest
> 
> modify data on ```stock_candy```
> 
> ```stock_candy.sh``` 
```
└─ [←[32mINFO ←[0m] ←[32m Added 50000 ALPHAGUM candy, ALG symbol @2XRD price

├─ ResourceDef: 0378a3b15108515504b4a9682eaaa43d4b13417ce6840fb5bf1fa2

└─ ResourceDef: 03570bd52401c8b3e6a6e551549f64199cc5c629726627e83211e1
```

---

>method ```stock_candy```  arguments ```50000,$BTG 1.5```

> cd transaction_manifest
> 
> modify data on ```stock_candy```
> 
> ```stock_candy.sh```  
```
└─ [←[32mINFO ←[0m] ←[32m Added 50000 BETAGUM candy, BTG symbol @1.5XRD price

├─ ResourceDef: 032387943b7cd89d99ee07d672fd9945029c99300282931690ddab

└─ ResourceDef: 033af09cc79097add03aa9614eadb005e61874681545a1ac2b8caf
```

---
>resim show $Default-account
```
├─ { amount: 1000000, resource_def: $XRD, name: "Radix", symbol: "XRD" }

├─ { amount: 50000, resource_def: $BTG, name: "BETAGUM", symbol: "BTG" }

└─ { amount: 50000, resource_def: $ALG, name: "ALPHAGUM", symbol: "ALG" }
```
-----------------------------------------------------------------------------------------------------------------------
#
### part_2_1
Test "get_xrd_sell_amount_becsx" method coupled with "buy_exact_candy_sell_xrd" method & check default-account balances
-----------------------------------------------------------------------------------------------------------------------

>method ```get_xrd_sell_amount_becsx```  arguments ```$ALG 5000```

> cd transaction_manifest
> 
> modify data on ```get_xrd_sell_amount_becsx```
> 
> ```get_xrd_sell_amount_becsx.sh```  
```
├─ Decimal("11235.955056179775277776")
```

---
>resim call-method $CandyDex buy_exact_candy_sell_xrd 5000 $ALG 11235.95505617977527778,$XRD


---
>resim show $Default-account

-11235.95505617977527778
```
├─ { amount: 988764.04494382022472222, resource_def: $XRD, name: "Radix", symbol: "XRD" }

├─ { amount: 55000, resource_def: $ALG, name: "ALPHAGUM", symbol: "ALG" }
```
+5000

----------------------------------------------

>method ```get_xrd_sell_amount_becsx```  arguments ```$BTG 5000```

> cd transaction_manifest
> 
> modify data on ```get_xrd_sell_amount_becsx```
> 
> ```get_xrd_sell_amount_becsx.sh```  
```
├─ Decimal("8426.966292134831459595")
```

---

>method ```buy_exact_candy_sell_xrd```  arguments ```5000 $BTG 8426.966292134831459597,$XRD```

> cd transaction_manifest
> 
> modify data on ```buy_exact_candy_sell_xrd```
> 
> ```buy_exact_candy_sell_xrd.sh```  


---
>resim show $Default-account

+5000
```
├─ { amount: 55000, resource_def: $BTG, name: "BETAGUM", symbol: "BTG" }

├─ { amount: 980337.078651685393262624, resource_def: $XRD, name: "Radix", symbol: "XRD" }
```
-8426.966292134831459596

---
>P.S. Due to calculation approximation , to obtain the exact output amount required, sometimes it could required to round in excess last numbers of the fractional part >beyond dot (17/18).

------------------------------------------------------------------------------------------------------------------------
#
### part_2_2
Test "get_candy_buy_amount_bcsex" method coupled with "buy_candy_sell_exact_xrd" method & check default-account balances
------------------------------------------------------------------------------------------------------------------------

>method ```get_candy_buy_amount_bcsex```  arguments ```$ALG 10000```

> cd transaction_manifest
> 
> modify data on ```get_candy_buy_amount_bcsex```
> 
> ```get_candy_buy_amount_bcsex.sh```  

```
├─ Decimal("4045.867346938775511593")
```

---

>method ```buy_candy_sell_exact_xrd```  arguments ```4045.867346938775511593 $ALG 10000,$XRD```

> cd transaction_manifest
> 
> modify data on ```buy_candy_sell_exact_xrd```
> 
> ```buy_candy_sell_exact_xrd.sh```  


---
>resim show $Default-account

-10000
```
├─ { amount: 970337.078651685393262624, resource_def: $XRD, name: "Radix", symbol: "XRD" }

├─ { amount: 59045.867346938775511593, resource_def: $ALG, name: "ALPHAGUM", symbol: "ALG" }
```
+4045.867346938775511593

----------------------------------------------

>method ```get_candy_buy_amount_bcsex```  arguments ```$BTG 10000```

> cd transaction_manifest
> 
> modify data on ```get_candy_buy_amount_bcsex```
> 
> ```get_candy_buy_amount_bcsex.sh```  


├─ Decimal("5235.985473753714098268")

		    				     
---
>resim call-method $CandyDex buy_candy_sell_exact_xrd 5235.985473753714098268 $BTG 10000,$XRD


---
>resim show $Default-account

+5235.985473753714098268
```
├─ { amount: 60235.985473753714098268, resource_def: $BTG, name: "BETAGUM", symbol: "BTG" }

├─ { amount: 960337.078651685393262624, resource_def: $XRD, name: "Radix", symbol: "XRD" }
```
-10000

-----------------------------------------------------------------------------------------------------------------------
#
### part_2_3
Test "get_candy_sell_amount_bexsc" method coupled with "buy_exact_xrd_sell_candy" method & check default-account balances
-----------------------------------------------------------------------------------------------------------------------

>method ```get_candy_sell_amount_bexsc```  arguments ```$ALG 5000```

> cd transaction_manifest
> 
> modify data on ```get_candy_sell_amount_bexsc```
> 
> ```get_candy_sell_amount_bexsc.sh```  

```
├─ Decimal("2173.55005206646298378")
```

---

>method ```buy_exact_xrd_sell_candy```  arguments ```5000 2173.55005206646298378,$ALG```

> cd transaction_manifest
> 
> modify data on ```buy_exact_xrd_sell_candy```
> 
> ```buy_exact_xrd_sell_candy.sh```

---
>resim show $Default-Account

+5000
```
├─ { amount: 965337.078651685393262624, resource_def: $XRD, name: "Radix", symbol: "XRD" }

├─ { amount: 56872.317294872312527813, resource_def: $ALG, name: "ALPHAGUM", symbol: "ALG" }
```
-2173.55005206646298378

----------------------------------------------------------------

>method ```get_candy_sell_amount_bexsc```  arguments ```$BTG 5000```

> cd transaction_manifest
> 
> modify data on ```get_candy_sell_amount_bexsc```
> 
> ```get_candy_sell_amount_bexsc.sh```

```
├─ Decimal("2863.504580183464650221")
```

---

>method ```buy_exact_xrd_sell_candy```  arguments ```5000 2863.504580183464650221,$BTG```

> cd transaction_manifest
> 
> modify data on ```buy_exact_xrd_sell_candy```
> 
> ```buy_exact_xrd_sell_candy.sh```



---
>resim show $Default-Account

+5000
```
├─ { amount: 970337.078651685393262624, resource_def: $XRD, name: "Radix", symbol: "XRD" }

├─ { amount: 57372.480893570249448047, resource_def: $BTG, name: "BETAGUM", symbol: "BTG" }
```
-2863,504580183464650221

-----------------------------------------------------------------------------------------------------------------------
#
### part_2_4
Test "get_xrd_buy_amount_bxsec" method coupled with "buy_xrd_sell_exact_candy" method & check default-account balances
-----------------------------------------------------------------------------------------------------------------------

>method ```get_xrd_buy_amount_bxsec```  arguments ```$ALG 3000```

> cd transaction_manifest
> 
> modify data on ```get_xrd_buy_amount_bxsec```
> 
> ```get_xrd_buy_amount_bxsec.sh```

```
├─ Decimal("6452.32299588470424501")
```

--- 
>method ```buy_xrd_sell_exact_candy```  arguments ```6452.32299588470424501 3000,$ALG```

> cd transaction_manifest
> 
> modify data on ```buy_xrd_sell_exact_candy```
> 
> ```buy_xrd_sell_exact_candy.sh```


---
>resim show $Default-Account

+6452,322995884704244423
```
├─ { amount: 976789.401647570097507047, resource_def: $XRD, name: "Radix", symbol: "XRD" }

├─ { amount: 53872.317294872312527813, resource_def: $ALG, name: "ALPHAGUM", symbol: "ALG" }
```
-3000

-----------------------------------------------------------------

>method ```get_xrd_buy_amount_bxsec```  arguments ```$BTG 3000```

> cd transaction_manifest
> 
> modify data on ```get_xrd_buy_amount_bxsec```
> 
> ```get_xrd_buy_amount_bxsec.sh```
```
├─ Decimal("4893.91676935180419015")
```

---

>method ```buy_xrd_sell_exact_candy```  arguments ```4893.91676935180419015 3000,$BTG```

> cd transaction_manifest
> 
> modify data on ```buy_xrd_sell_exact_candy```
> 
> ```buy_xrd_sell_exact_candy.sh```



---
>resim show $Default-Account

+4893,916769351804189387
```
├─ { amount: 981683.318416921901696434, resource_def: $XRD, name: "Radix", symbol: "XRD" }

├─ { amount: 54372.480893570249448047, resource_def: $BTG, name: "BETAGUM", symbol: "BTG" }
```
-3000 

>P.S. Due to calculation approximation, protocol return an output amount in defect by last numbers of the fractional part beyond dot (15/16).

--------------------------------------------------------------------------------------------------------------------------
#
### part_2_5
Test "get_candy_buy_amount_bcsec" method coupled with "buy_candy_sell_exact_candy" method & check default-account balances
--------------------------------------------------------------------------------------------------------------------------

>resim show $Default-Account
```
├─ { amount: 53872.317294872312527813, resource_def: $ALG, name: "ALPHAGUM", symbol: "ALG" }

├─ { amount: 54372.480893570249448047, resource_def: $BTG, name: "BETAGUM", symbol: "BTG" }
```
---

>method ```get_candy_buy_amount_bcsec```  arguments ```$ALG 5000 $BTG```

> cd transaction_manifest
> 
> modify data on ```get_candy_buy_amount_bcsec```
> 
> ```get_candy_buy_amount_bcsec.sh```

```
├─ Decimal("3501.579327852255786347")
```

---

>method ```buy_candy_sell_exact_candy```  arguments ```3501.579327852255786347 $ALG 5000,$BTG```

> cd transaction_manifest
> 
> modify data on ```buy_candy_sell_exact_candy```
> 
> ```buy_candy_sell_exact_candy.sh```



---
>resim show $Default-Account

+3501.579327852255786347
```
├─ { amount: 57373.89662272456831416, resource_def: $ALG, name: "ALPHAGUM", symbol: "ALG" }

├─ { amount: 49372.480893570249448047, resource_def: $BTG, name: "BETAGUM", symbol: "BTG" }
```
-5000

------------------------------------------------------------------

>method ```get_candy_buy_amount_bcsec```  arguments ```$BTG 5000 $ALG```

> cd transaction_manifest
> 
> modify data on ```get_candy_buy_amount_bcsec```
> 
> ```get_candy_buy_amount_bcsec.sh```

```
├─ Decimal("5825.932438133595144962")
```

---

>method ```buy_candy_sell_exact_candy```  arguments ```5825.932438133595144962 $BTG 5000,$ALG```

> cd transaction_manifest
> 
> modify data on ```buy_candy_sell_exact_candy```
> 
> ```buy_candy_sell_exact_candy.sh```

---
>resim show $Default-Account

-5000
```
├─ { amount: 52373.89662272456831416, resource_def: $ALG, name: "ALPHAGUM", symbol: "ALG" }

├─ { amount: 55198.413331703844593009, resource_def: $BTG, name: "BETAGUM", symbol: "BTG" }
```
+5825.932438133595144962

--------------------------------------------------------------------------------------------------------------------------
#
### part_2_6
Test "get_candy_sell_amount_becsc" method coupled with "buy_exact_candy_sell_candy" method & check default-account balances 
--------------------------------------------------------------------------------------------------------------------------

>method ```get_candy_sell_amount_becsc```  arguments ```5000 $ALG $BTG```

> cd transaction_manifest
> 
> modify data on ```get_candy_sell_amount_becsc```
> 
> ```get_candy_sell_amount_becsc.sh```

```
├─ Decimal("7374.181373608690958584")
```

---

>method ```buy_exact_candy_sell_candy```  arguments ```5000 $ALG 7374.181373608690958584,$BTG```

> cd transaction_manifest
> 
> modify data on ```buy_exact_candy_sell_candy```
> 
> ```buy_exact_candy_sell_candy.sh```


---
>resim show $Default-Account

+5000
```
├─ { amount: 57373.89662272456831416, resource_def: $ALG, name: "ALPHAGUM", symbol: "ALG" }

├─ { amount: 47824.231958095153634425, resource_def: $BTG, name: "BETAGUM", symbol: "BTG" }
```
-7374,181373608690958584

------------------------------------------------------------------

>method ```get_candy_sell_amount_becsc```  arguments ```5000 $BTG $ALG```

> cd transaction_manifest
> 
> modify data on ```get_candy_sell_amount_becsc```
> 
> ```get_candy_sell_amount_becsc.sh```

```
├─ Decimal("4198,80347228503780403")
```

---

>method ```buy_exact_candy_sell_candy```  arguments ```5000 $BTG 4198,80347228503780403,$ALG```

> cd transaction_manifest
> 
> modify data on ```buy_exact_candy_sell_candy```
> 
> ```buy_exact_candy_sell_candy.sh```


---
>resim show $Default-Account

-4198,80347228503780403
```
├─ { amount: 53175.09315043953051013, resource_def: $ALG, name: "ALPHAGUM", symbol: "ALG" }

├─ { amount: 52824.231958095153634425, resource_def: $BTG, name: "BETAGUM", symbol: "BTG" }
```
+5000

[Back Up](#index)
#
### Part_3
# Let's test "flashswap" method.
-------------------------------------------------------------------------------------------
>A.S.: Method testable with a Dummy DEX Blueprint findable at this address:
>
>[![Github](https://img.shields.io/badge/Github-Alanci17-blueviolet.svg)](https://github.com/alanci17/radix-scrypto/blob/main/dummydex/src/lib.rs)

----------------------------------------------------------------------------------------------------------
Simulator reset & new Default-account generation
----------------------------------------------------------------------------------------------------------

>resim reset
>
>resim new-account 
```
├─ Account address: 02ffa01926302c78c0f050f6d08140ec77757ec6cd277f7eecef42 = $Default-account

└─ Public key: 005feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9 = $Public-key
```
----------------------------------------------------------------------------------------------------------
Publish DummyDex Blueprint & Component instantiate 
----------------------------------------------------------------------------------------------------------

>resim publish .
```
└─ Package: 018c890168ca8b6702cc8c1ce7030d65667390bdea7661767df694 = $Package-DummyDex
```
---

>function ```new```  

> cd DummyDex/transaction_manifest
> 
> modify data on ```dummydex_instantiate```
> 
> ```dummydex_instantiate.sh```
```
└─ Component: 026c202008c0bc5323a2b57e409b4ffff0a8c30d7ce12645b21806 = $DummyDex
```

----------------------------------------------------------------------------------------------------------
Publish CandyDex Blueprint & Component instantiate 
----------------------------------------------------------------------------------------------------------

>resim publish .
```
└─ Package: 013fa22e238526e9c82376d2b4679a845364243bf970e5f783d13f = $Package-CandyDex
```
---
>resim call-function $Package-CandyDex CandyDex new 1 

>function ```new```  arguments ```1``` 

> cd CandyDex/transaction_manifest
> 
> modify data on ```candydex_instantiate```
> 
> ```candydex_instantiate.sh```

```
├─ ResourceDef: 03eb23d0867f32265935d93970aded9033cc868d31795f27d8cb62 = $MinterBadge

├─ ResourceDef: 03d527faee6d0b91e7c1bab500c6a986e5777a25d704acc288d542 = $OwnerBadge

└─ Component: 02ac00a15a87df7c43b55e49d5d229bc770136c108586a9d7b38b5	= $CandyDex
```
----------------------------------------------------------------------------------------------------------
Create some candy resources 
----------------------------------------------------------------------------------------------------------	

>resim new-token-fixed --name "GAMMAGUM" 100000 --symbol "GMG"
```
└─ ResourceDef: 037395fc4a92210f3c576bd5e621a46f49643ef9b9e093828874e8 = $GMG
```

---
>resim new-token-fixed --name "DELTAGUM" 100000 --symbol "DTG"
```
└─ ResourceDef: 03d1f50010e4102d88aacc347711491f852c515134a9ecf67ba17c = $DTG
```

---
>resim new-token-fixed --name "SIGMAGUM" 100000 --symbol "SGG"
```
└─ ResourceDef: 03c29248a0d4c7d4da7b323adfeb4b4fbe811868eb637725ebb7c1 = $SGG
```

----------------------------------------------------------------------------------------------------------
Stock candies resources in CandyDex Blueprint from Default-account 
----------------------------------------------------------------------------------------------------------

>method ```stock_candy```  arguments ```20000,$GMG 2```

> cd transaction_manifest
> 
> modify data on ```stock_candy```
> 
> ```stock_candy.sh```


---

>method ```stock_candy```  arguments ```20000,$DTG 2```

> cd transaction_manifest
> 
> modify data on ```stock_candy```
> 
> ```stock_candy.sh```


---

>method ```stock_candy```  arguments ```20000,$SGG 1.5```

> cd transaction_manifest
> 
> modify data on ```stock_candy```
> 
> ```stock_candy.sh```


----------------------------------------------------------------------------------------------------------
Buy some candies to make some $XRD flowing into CandyDex Blueprint from Default-account and check balances 
----------------------------------------------------------------------------------------------------------	

>method ```buy_exact_candy_sell_xrd```  arguments ```2000 $DTG 5000,$XRD```

> cd transaction_manifest
> 
> modify data on ```buy_exact_candy_sell_xrd```
> 
> ```buy_exact_candy_sell_xrd.sh```



---

>method ```buy_exact_candy_sell_xrd```  arguments ```2000 $DTG 7000,$XRD```

> cd transaction_manifest
> 
> modify data on ```buy_exact_candy_sell_xrd```
> 
> ```buy_exact_candy_sell_xrd.sh```


---
>resim show $CandyDex
```
├─ { amount: 9556.95393400474438182, resource_def: $XRD, name: "Radix", symbol: "XRD" }

├─ { amount: 20000, resource_def: $SGG, name: "SIGMAGUM", symbol: "SGG" }

├─ { amount: 20000, resource_def: $GMG, name: "GAMMAGUM", symbol: "GMG" }

├─ { amount: 16000, resource_def: $DTG, name: "DELTAGUM", symbol: "DTG" }

└─ { amount: 1, resource_def: $MinterBadge, name: " MinterBadge " }
```
----------------------------------------------------------------------------------------------------------
Stock candies resources in DummyDex Blueprint from Default-account 
----------------------------------------------------------------------------------------------------------

>method ```stock_candy```  arguments ```10000,$GMG 2```

> cd DummyDex/transaction_manifest
> 
> modify data on ```stock_candy```
> 
> ```stock_candy.sh```


---

>method ```stock_candy```  arguments ```10000,$DTG 1.5```

> cd DummyDex/transaction_manifest
> 
> modify data on ```stock_candy```
> 
> ```stock_candy.sh```

---

>method ```stock_candy```  arguments ```10000,$SGG 2```

> cd DummyDex/transaction_manifest
> 
> modify data on ```stock_candy```
> 
> ```stock_candy.sh```

----------------------------------------------------------------------------------------------------------
Transfer some $XRD resources to DummyDex Blueprint from Default-account and check balances
----------------------------------------------------------------------------------------------------------

>method ```put_xrd```  arguments ```10000,$XRD```

> cd DummyDex/transaction_manifest
> 
> modify data on ```put_xrd```
> 
> ```put_xrd.sh```

---
>resim show $DummyDex
```
├─ { amount: 10000, resource_def: $XRD, name: "Radix", symbol: "XRD" }

├─ { amount: 10000, resource_def: $SGG, name: "SIGMAGUM", symbol: "SGG" }

├─ { amount: 10000, resource_def: $GMG, name: "GAMMAGUM", symbol: "GMG" }

└─ { amount: 10000, resource_def: $DTG, name: "DELTAGUM", symbol: "DTG" }
```
----------------------------------------------------------------------------------------------------------
## Check balances, Call "flashswap" method on CandyDex Blueprint & verify amounts
#
### part_3_1
Borrow XRD & reimburse XRD
----------------------------------------------------------------------------------------------------------
>resim show $CandyDex
```
├─ { amount: 9556.95393400474438182, resource_def: $XRD, name: "Radix", symbol: "XRD" }
```
>resim show $DummyDex
```
├─ { amount: 10000, resource_def: $XRD, name: "Radix", symbol: "XRD" }
```
>resim show $Defaul-account
```
├─ { amount: 980443.04606599525561818, resource_def: $XRD, name: "Radix", symbol: "XRD" }
```

>method ```flashswap```  arguments ```100 $XRD $XRD $DummyDex "arb_dex"```

> cd CandyDex/transaction_manifest
> 
> modify data on ```flashswap```
> 
> ```flashswap.sh```



---
>resim show $CandyDex
```
├─ { amount: 9557.95393400474438182, resource_def: $XRD, name: "Radix", symbol: "XRD" }		 +1.000000000000000000
```
>resim show $DummyDex
```
├─ { amount: 9990, resource_def: $XRD, name: "Radix", symbol: "XRD" }			        -10.000000000000000000
```
>resim show $Defaul-account
```
├─ { amount: 980452.04606599525561818, resource_def: $XRD, name: "Radix", symbol: "XRD" }	 +9.000000000000000000
```
----------------------------------------------------------------------------------------------------------
#
### part_3_2
Borrow Candy & reimburse a different Candy
----------------------------------------------------------------------------------------------------------

>resim show $CandyDex
```
├─ { amount: 20000, resource_def: $GMG, name: "GAMMAGUM", symbol: "GMG" }

├─ { amount: 16000, resource_def: $DTG, name: "DELTAGUM", symbol: "DTG" }
```
>resim show $DummyDex
```
├─ { amount: 10000, resource_def: $GMG, name: "GAMMAGUM", symbol: "GMG" }

└─ { amount: 10000, resource_def: $DTG, name: "DELTAGUM", symbol: "DTG" }
```
>resim show $Defaul-account
```
├─ { amount: 74000, resource_def: $DTG, name: "DELTAGUM", symbol: "DTG" }

├─ { amount: 70000, resource_def: $GMG, name: "GAMMAGUM", symbol: "GMG" }
```
---

>method ```flashswap```  arguments ```100 $GMG $DTG $DummyDex "arb_dex"```

> cd CandyDex/transaction_manifest
> 
> modify data on ```flashswap```
> 
> ```flashswap.sh```


---
>resim show $CandyDex
```
├─ { amount: 19900, resource_def: $GMG, name: "GAMMAGUM", symbol: "GMG" }		        -100.000000000000000000

├─ { amount: 16080.607409674749742129, resource_def: $DTG, name: "DELTAGUM", symbol: "DTG" }    +80.607409674749742129
```
>resim show $DummyDex
```
├─ { amount: 10100, resource_def: $GMG, name: "GAMMAGUM", symbol: "GMG" }		       +100.000000000000000000

└─ { amount: 9866.666666666666666667, resource_def: $DTG, name: "DELTAGUM", symbol: "DTG" }    -133.333333333333333333
```
>resim show $Defaul-account
```
├─ { amount: 70000, resource_def: $GMG, name: "GAMMAGUM", symbol: "GMG" }		         +0.000000000000000000

├─ { amount: 74052.725923658583591204, resource_def: $DTG, name: "DELTAGUM", symbol: "DTG" }    +52.725923658583591204
```
----------------------------------------------------------------------------------------------------------
#
### part_3_3
Borrow Candy & reimburse XRD
----------------------------------------------------------------------------------------------------------

>resim show $CandyDex
```
├─ { amount: 9557.95393400474438182, resource_def: $XRD, name: "Radix", symbol: "XRD" }

├─ { amount: 20000, resource_def: $SGG, name: "SIGMAGUM", symbol: "SGG" }
```
>resim show $DummyDex
```
├─ { amount: 9990, resource_def: $XRD, name: "Radix", symbol: "XRD" }

├─ { amount: 10000, resource_def: $SGG, name: "SIGMAGUM", symbol: "SGG" }
```
>resim show $Defaul-account
```
├─ { amount: 980452.04606599525561818, resource_def: $XRD, name: "Radix", symbol: "XRD" }

└─ { amount: 70000, resource_def: $SGG, name: "SIGMAGUM", symbol: "SGG" }
```

>method ```flashswap```  arguments ```100 $SGG $XRD $DummyDex "arb_dex"```

> cd CandyDex/transaction_manifest
> 
> modify data on ```flashswap```
> 
> ```flashswap.sh```

---
>resim show $CandyDex
```
├─ { amount: 9709.45393400474438182, resource_def: $XRD, name: "Radix", symbol: "XRD" }     +151.500000000000000000
                                                                                           
├─ { amount: 19900, resource_def: $SGG, name: "SIGMAGUM", symbol: "SGG" }		    -100.000000000000000000
```
>resim show $DummyDex
```
├─ { amount: 9790, resource_def: $XRD, name: "Radix", symbol: "XRD" }		            -200.000000000000000000

├─ { amount: 10100, resource_def: $SGG, name: "SIGMAGUM", symbol: "SGG" }		     +10.000000000000000000
```
>resim show $Defaul-account
```
├─ { amount: 70000, resource_def: $SGG, name: "SIGMAGUM", symbol: "SGG" }		      +0.000000000000000000

├─ { amount: 980500.54606599525561818, resource_def: $XRD, name: "Radix", symbol: "XRD" }    +48.500000000000000000
```
----------------------------------------------------------------------------------------------------------
#
### part_3_4
Example of reverted transaction due to unprofitable "flashswap" method call
----------------------------------------------------------------------------------------------------------

>method ```flashswap```  arguments ```100 $DTG $GMG $DummyDex "arb_dex""```

> cd CandyDex/transaction_manifest
> 
> modify data on ```flashswap```
> 
> ```flashswap.sh```

```
└─ [←[32mINFO ←[0m] ←[32m Sorry mate, ain't nothin' to scrape!
```
Error: TransactionExecutionError(InvokeError(Trap(Trap { kind: Unreachable })))


[Back Up](#index)
