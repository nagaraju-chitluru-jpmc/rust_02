#[derive(Debug, PartialEq)]
enum PaymentType {
    DigitalToken,
    Cash,
}

struct Seller {
    payment_type: PaymentType,
    price: f32,
    balance: f32,
}

#[derive(Debug)]
struct Buyer {
    name: String,
    payment_type: PaymentType,
    balance: f32,
}

struct BuyerGroup {
    members: Vec<Buyer>,
}

impl BuyerGroup {
    fn add_member(&mut self, h: Buyer) {
        self.members.push(h);
    }

    fn find_buyer(&self, payment_type: &PaymentType) -> i32 {
        println!("Searching for Buyer with payment_type {:?}", payment_type);
        let mut pos = 0;
        for i in &self.members {
            //use & else error: move occurs because `self.members` has type `Vec<Buyer>
            if i.payment_type == *payment_type {
                //dereference to point to actual value
                println!(
                    "Matching Buyer using PaymentType {:?} was found at index {}",
                    payment_type, pos
                );
                println!("{:?}", i);
                return pos;
            }
            pos = pos + 1;
        }
        println!("Buyer with PaymentType {:?} NOT found", payment_type);
        return -1;
    }

    fn buy(&mut self, buyer_index: i32, seller: &mut Seller) {
        let mut buyer = &mut self.members[buyer_index as usize]; // get reference to buyer
        loop {
            //Buyer buy from seller
            if buyer.balance >= seller.price {
                seller.balance += seller.price;
                buyer.balance -= seller.price;
                println!(
                    "{} balance: {}. Seller balance: {}",
                    buyer.name, buyer.balance, seller.balance
                );
            } else {
                println!(
                    "{} balance {} insufficient. Seller balance: {}",
                    buyer.name, buyer.balance, seller.balance
                );
                break;
            }
        }
    }
}

fn main() {
    let buyer1 = Buyer {
        name: "John".to_owned(),
        payment_type: PaymentType::DigitalToken,
        balance: 100.00,
    };
    let buyer2 = Buyer {
        name: "Sally".to_owned(),
        payment_type: PaymentType::Cash,
        balance: 100.00,
    };

    let mut buyer_group = BuyerGroup {
        members: Vec::new(),
    };

    buyer_group.add_member(buyer1);
    buyer_group.add_member(buyer2);

    let mut seller = Seller {
        payment_type: PaymentType::Cash,
        price: 10.0,
        balance: 0.0,
    };

    let buyer_index = buyer_group.find_buyer(&seller.payment_type);
    if buyer_index >= 0 {
        buyer_group.buy(buyer_index, &mut seller);
    }
}
