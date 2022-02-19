use scrypto::prelude::*;

blueprint! {
    struct DummyDex {
        flash_xrd: Vault,
        flash_vault: HashMap<Address, Vault>,
        flash_price: HashMap<Address, Decimal>
    }

    impl DummyDex {
        pub fn new() -> Component {
            Self { 
                flash_xrd: Vault::new(RADIX_TOKEN),
                flash_vault: HashMap::new(),
                flash_price: HashMap::new()
            }
            .instantiate()
        } 

        pub fn put_xrd(&mut self, xrd_bckt: Bucket) {
            self.flash_xrd.put(xrd_bckt);
        }

        pub fn stock_candy(&mut self, candy: Bucket, new_price: Decimal) {
            let candy_addr = candy.resource_address();
            let v = self.flash_vault.entry(candy.resource_address())
                .or_insert(Vault::new(candy.resource_address()));
            v.put(candy);
            self.flash_price.insert(candy_addr, new_price);
        } 

        pub fn arb_dex(&mut self, token: Bucket, token_out_addr: Address ) -> Bucket {
                let token_bucket;
                let price_in: Decimal;
                let amount = token.amount();                     
                let address = token.resource_address();
                if token.resource_address() == token_out_addr {                             
                    if token.resource_address() == self.flash_xrd.resource_address() {
                        self.flash_xrd.put(token);
                        let token_out = amount+amount/10;  
                        token_bucket = self.flash_xrd.take(*&token_out); 
                    } else {
                        let token_out = amount+amount/10;
                        token_bucket = match self.flash_vault.get_mut(&address) {
                            Some(v) => {
                                v.put(token);
                                v.take(token_out)
                            }
                            None => std::process::abort()                  
                        };
                    }
                } else {
                    if token.resource_address() == self.flash_xrd.resource_address() {
                        self.flash_xrd.put(token);     
                        price_in = dec!(1);
                    } else {
                        match self.flash_vault.get_mut(&address) {
                            Some(v) =>  v.put(token),
                            None => std::process::abort()                  
                        };
                        price_in = DummyDex::flashprice(self, address); 
                    }
                    if token_out_addr != self.flash_xrd.resource_address() {
                        let price_out: Decimal = DummyDex::flashprice(self, token_out_addr); 
                        let token_out_nbr = amount*price_in/price_out;  
                        token_bucket = match self.flash_vault.get_mut(&token_out_addr) {
                            Some(v) => v.take(token_out_nbr),    
                            None => { info!("Candy not in stock !"); std::process::abort() }
                        };
                    } else {
                        let price_out: Decimal =  1i32.into(); 
                        let token_out_nbr = amount*price_in/price_out;  
                        token_bucket = self.flash_xrd.take(*&token_out_nbr); 
                    }
                }            
                token_bucket
        }

            fn flashprice(&mut self, candy_out_addr: Address) -> Decimal {
                let price: Decimal;  
                let _price = match self.flash_price.get(&candy_out_addr) {
                    Some(_price) => price = *_price,
                    None => { info!("Candy not in stock !");
                        std::process::abort()
                    }
                };
                price
            } 
    }
}   








