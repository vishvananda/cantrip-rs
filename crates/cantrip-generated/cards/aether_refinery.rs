// Aether Refinery
// status: ready
// If you would get one or more {E} (energy counters), you get twice that many {E} instead.
// {T}: You get {E}, then you may pay one or more {E}. If you do, create an X/X black Aetherborn creature token, where X is the amount of {E} paid this way.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::PlayerCounters(c) = proposal { let subject = Target::Player(c.player); let data = [u32::from(c.energy), u32::MAX]; if c.energy > 0 && Some(asker.controller) == Some(c.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::PlayerCounters(seen) = &seen { let c = seen; let subject = Target::Player(c.player); let data = [u32::from(c.energy), u32::MAX]; if let Proposal::PlayerCounters(c) = proposal { c.energy = 0; } } }
#[allow(unused_variables, unused_mut)] fn ability_0_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let amount_0 = { let q: &dyn Query = g; ((2i64 * i64::from(data[0]))).clamp(0, 2147483647) as i32 }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.change_energy(player_0, amount_0);
 Ok(()) }
#[allow(unused_variables)] fn ability_0_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::PlayerCounters(c) = before { let q: &dyn Query = g; let subject = Target::Player(c.player); let data = [u32::from(c.energy), u32::MAX]; let _ = ability_0_then_steps(g, asker, subject, data); } }
pub mod token_5 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Aetherborn).name, token_words: token!(Aetherborn).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Aetherborn]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.change_energy(player_1, amount_1);
g.end_sentence();
let payer_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_2 = cantrip_mechanics::effects::ask_may(g, payer_2, asker.source, Purpose::Optional)?; if paid_2 {
let mut done_3: i64 = 0; for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { done_3 += { let mut paid = 0i64; while g.state().player(player).energy() > 0 && cantrip_mechanics::effects::ask_may(g, player, asker.source, Purpose::Optional)? { g.change_energy(player, -1); paid += 1; } paid }; }
g.end_sentence();
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_2 as i64) != 0) } {
let count_4 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let power_4 = { let q: &dyn Query = g; ((paid_2 as i64)).clamp(0, 32767) as i16 }; let toughness_4 = { let q: &dyn Query = g; ((paid_2 as i64)).clamp(0, 32767) as i16 }; let players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut values_4 = Copiable::of(g.card_id(&token_5::CARD)).sized(power_4, toughness_4); values_4.words = g.words(asker.source); let mut created_4: Vec<ObjectId> = Vec::new(); for player in players_4 { created_4.extend(g.create_token_copies_of(values_4, player, count_4)?); }
g.end_sentence();
hoisted_6.extend(created_4.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Aether Refinery", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{T}: You get {E}, then you may pay one or more {E}. If you do, create an X/X black Aetherborn creature token, where X is the amount of {E} paid this way.").targets(&[])], statics: &[], replacements: &[replacement(ReplacementKind::Counters, ability_0_applies, ability_0_apply, "If you would get one or more {E} , you get twice that many {E} instead.").then(ability_0_then)], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_5::CARD];
