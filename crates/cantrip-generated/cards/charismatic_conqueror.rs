// Charismatic Conqueror
// status: ready
// Vigilance
// Whenever an artifact or creature an opponent controls enters untapped, they may tap that permanent. If they don't, you create a 1/1 white Vampire creature token with lifelink.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && (((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature)) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))) && !(q.state().obj(id).tapped)) { return Some(Fired { subject, data }); } }
 None }
pub mod token_2 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Vampire).name, token_words: token!(Vampire).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Vampire]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; item.subject.object().map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") }; let paid_0 = match { let q: &dyn Query = g; item.subject.object() }.filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield) && !g.state().obj(o).tapped) { Some(o) if cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)? => { g.tap(o); true } _ => false };
g.end_sentence();
if { let q: &dyn Query = g; !(((paid_0 as i64) != 0)) } {
let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_1 = Vec::new();
let count_1_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_1_0 = g.card_id(&token_2::CARD);
for player in players_1 {
tokens_1.extend(std::iter::repeat((def_1_0,player)).take(count_1_0 as usize));
} let words_1 = g.words(asker.source);
let created_1 = g.create_tokens_worded(&tokens_1, false, None, words_1)?;
g.end_sentence();
hoisted_3.extend(created_1.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Charismatic Conqueror", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Vampire,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "Whenever an artifact or creature an opponent controls enters untapped, they may tap that permanent. If they don't, you create a 1/1 white Vampire creature token with lifelink.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_2::CARD];
