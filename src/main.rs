#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}


//una vez que account toma la referencia imprime account.balance y la referencia se elimina.
fn print_balance(account: &Account) {
    println!("{}", account.balance);
}
 
fn main() {
    let mut bank = Bank::new();
    let account = Account::new(1, String::from("me"));
 
    // Comentamos 'account_ref' debido a una violación de regla de ownership and borrowing.
    //"no puede haber ref a un valor cuando su propietario esta fuera de scope"
    //"referencia a un valor no pueden sobrevivir al valor al que refieren"
    //"Cuando un "owner" esta fuera de scope, el "valor que posee" es dropped( cleaned up in memory)"
    // let account_ref = &account;
 
    // Create the ref
    print_balance(&account);
    //cuando se mueve el valor al vector ya no hay referencia que apunte a account.
    bank.accounts.push(account);
}


