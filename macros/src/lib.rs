use proc_macro2::Span;
use quote::quote;
use syn::Ident;

// fn import_my_crate() -> TokenStream {
//     let found_crate =
//         crate_name("utils").expect("utils is present in `Cargo.toml`");

//     match found_crate {
//         FoundCrate::Itself => quote!(crate),
//         FoundCrate::Name(name) => {
//             let ident = Ident::new(&name, Span::call_site());
//             quote!( #ident )
//         }
//     }
// }

fn natural_risks(digit: i32) -> proc_macro2::TokenStream {
    match digit {
        1 => quote!(NaturalPhenomenon),
        _ => panic!("natural_risks: {digit}"),
    }
}

fn technological_risks(digit: i32) -> proc_macro2::TokenStream {
    match digit {
        1 => quote!(UrbanFire),
        2 => quote!(EquipmentAndProductsFire),
        3 => quote!(TransportsFire),
        4 => quote!(Accident),
        5 => quote!(IndustrialAccident),
        _ => panic!("technological_risks: {digit}"),
    }
}

fn mixed_risks(digit: i32) -> proc_macro2::TokenStream {
    match digit {
        1 => quote!(RuralFire),
        3 => quote!(SecurityServicesStructuresCompromise),
        _ => panic!("mixed_risks: {digit}"),
    }
}

fn protection_and_assitance_of_goods_and_people(digit: i32) -> proc_macro2::TokenStream {
    match digit {
        1 => quote!(HealthAssistance),
        2 => quote!(LegalConflictsIntervention),
        3 => quote!(AssistanceAndPreservationOfHumanActivities),
        _ => panic!("protection_and_assitance_of_goods_and_people: {digit}"),
    }
}

fn operations_and_alert_states(digit: i32) -> proc_macro2::TokenStream {
    match digit {
        1 => quote!(Operations),
        _ => panic!("operations_and_alert_states: {digit}"),
    }
}

fn op_code_match_branch(code: i32) -> proc_macro2::TokenStream {
    let nat = code;
    let family_digit = nat / 1000;
    let family = family_digit * 1000;
    let group_digit = (nat - family) / 100;
    let group = group_digit * 100;
    let kind = nat - family - group;

    // let my_crate = import_my_crate();

    let (family_message, family_mod, group_message) = match family_digit {
        1 => (
            quote!(NaturalRisks),
            quote!(natural_risks),
            natural_risks(group_digit),
        ),
        2 => (
            quote!(TechnologicalRisks),
            quote!(technological_risks),
            technological_risks(group_digit),
        ),
        3 => (
            quote!(MixedRisks),
            quote!(mixed_risks),
            mixed_risks(group_digit),
        ),
        4 => (
            quote!(ProtectionAndAssitanceOfGoodsAndPeople),
            quote!(protection_and_assitance_of_goods_and_people),
            protection_and_assitance_of_goods_and_people(group_digit),
        ),
        9 => (
            quote!(OperationsAndAlertStates),
            quote!(operations_and_alert_states),
            operations_and_alert_states(group_digit),
        ),
        _ => (
            quote!(compile_error!("invalid code")),
            quote!(compile_error!("invalid code")),
            quote!(compile_error!("invalid code")),
        ),
    };
    let output = quote! {
        #code => OccurrenceKind {
            kind: Some(occurrence_kind::Kind::#family_message(#family_message {
                family: Some(#family_mod::Family::#group_message(
                    #group_message::try_from(#kind).expect(&format!("{}", #code)).into(),
                )),
            })),
        },
    };

    output
}

fn code_const(code: u16) -> proc_macro2::TokenStream {
    let nat = code;
    let family_digit = nat / 1000;
    let family = family_digit * 1000;
    let group_digit = (nat - family) / 100;
    let group = group_digit * 100;
    let kind = nat - family - group;

    let family_ident = Ident::new(&format!("C{family_digit}"), Span::call_site());
    let group_ident = Ident::new(&format!("C{group_digit}"), Span::call_site());
    let kind_ident = Ident::new(&format!("C{kind}"), Span::call_site());

    let family_ident_lower = Ident::new(&format!("c{family_digit}"), Span::call_site());

    let code_ident = Ident::new(&format!("C{code}"), Span::call_site());
    let code_const = quote! {
        pub const #code_ident: occurrence::v1::Kind = occurrence::v1::Kind {
            inner: Some(occurrence::v1::kind::Inner::#family_ident(occurrence::v1::#family_ident {
                inner: Some(occurrence::v1::#family_ident_lower::Inner::#group_ident(
                    occurrence::#family_ident_lower::v1::#group_ident::#kind_ident as i32,
                )),
            })),
        };
    };
    code_const
}

#[proc_macro]
pub fn code_consts(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let codes = [
        1101, 1103, 1105, 1107, 1109, 1111, 1113, 1119, 1125, 2101, 2107, 2109, 2111, 2113, 2115,
        2117, 2119, 2121, 2123, 2125, 2127, 2129, 2201, 2203, 2301, 2303, 2305, 2307, 2401, 2403,
        2405, 2407, 2409, 2411, 2413, 2415, 2417, 2419, 2421, 2423, 2425, 2501, 2503, 2505, 2507,
        2509, 2511, 3101, 3103, 3105, 3107, 3109, 3111, 3301, 3309, 3311, 3313, 3315, 3321, 3323,
        3325, 3327, 3329, 3331, 3333, 4111, 4113, 4201, 4203, 4207, 4209, 4327, 4329, 4331, 4333,
        4335, 4339, 9103,
    ];

    let mut code_consts = proc_macro2::TokenStream::new();

    for code in codes {
        code_consts.extend(code_const(code));
    }

    quote! {
    mod code_consts {
        use super::*;
        #code_consts
    }
    }
    .into()
}

#[proc_macro]
pub fn op_code_match(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let codes = [
        1101, 1103, 1105, 1107, 1109, 1111, 1113, 1119, 1125, 2101, 2107, 2109, 2111, 2113, 2115,
        2117, 2119, 2121, 2123, 2125, 2127, 2129, 2201, 2203, 2301, 2303, 2305, 2307, 2401, 2403,
        2405, 2407, 2409, 2411, 2413, 2415, 2417, 2419, 2421, 2423, 2425, 2501, 2503, 2505, 2507,
        2509, 2511, 3101, 3103, 3105, 3107, 3109, 3111, 3301, 3309, 3311, 3313, 3315, 3321, 3323,
        3325, 3327, 3329, 3331, 3333, 4111, 4113, 4201, 4203, 4207, 4209, 4327, 4329, 4331, 4333,
        4335, 4339, 9103,
    ];

    let mut match_arms = proc_macro2::TokenStream::new();

    for code in codes {
        match_arms.extend(op_code_match_branch(code));
    }
    let output = quote! {
        fn nat_to_msg(code: i32) -> anyhow::Result<OccurrenceKind> {
            let res = match code {
                #match_arms
                _ => anyhow::bail!("invalid code: {}", code),
            };
            return Ok(res);
        }
    };

    output.into()
}
