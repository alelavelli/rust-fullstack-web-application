use yew::{Html, function_component};

#[function_component(Admin)]
pub fn admin_component() -> Html {
    /*
    Bisogna fare una richiesta per verificare se l'utente sia admin
    Questa info non la si può salvare nel local storage perché non è
    sicuro.

    Se l'utente non è admin allora si porta a not found
    Nell'header si aggiunge anche un link the porta alla admin page
    quando l'utente è admin.
    */
    todo!()
}
