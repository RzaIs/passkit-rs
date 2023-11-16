use webauthn_rs::prelude::{Url, Uuid};
use webauthn_rs::WebauthnBuilder;

fn main() {

    let rp_id = "rza.is";
    let rp_origin = "https://passkit.rza.is";

    let rp_origin = &Url::parse(rp_origin)
        .expect("Url::parse");

    println!("rp_origin = Url::parse success");

    let webauthn_builder = WebauthnBuilder::new(rp_id, rp_origin)
        .map(|b| b.rp_name("passkit"))
        .expect("WebauthnBuilder::new");

    println!("webauthn_builder = WebauthnBuilder::new success");

    let webauthn = webauthn_builder.build().expect("webauthn_builder.build");

    println!("webauthn = webauthn_builder.build success");

    let user_uuid = Uuid::new_v4();

    _ = webauthn.start_passkey_registration(
        user_uuid,
        "rzais",
        "Rza Ismayilov",
        None
    ).expect("webauthn.start_passkey_registration");

    println!("webauthn.start_passkey_registration success");
}
