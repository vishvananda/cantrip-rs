// Liberated Livestock
// status: ready
// When this creature dies, create a 1/1 white Cat creature token with lifelink, a 1/1 white Bird creature token with flying, and a 2/4 white Ox creature token. For each of those tokens, you may put an Aura card from your hand and/or graveyard onto the battlefield attached to it.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Cat).name, token_words: token!(Cat).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Cat]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
pub mod token_2 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Bird).name, token_words: token!(Bird).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Bird]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
pub mod token_3 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Ox).name, token_words: token!(Ox).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Ox]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(2), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
let count_0_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_1 = g.card_id(&token_2::CARD);
let count_0_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_2 = g.card_id(&token_3::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
tokens_0.extend(std::iter::repeat((def_0_1,player)).take(count_0_1 as usize));
tokens_0.extend(std::iter::repeat((def_0_2,player)).take(count_0_2 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
for each_object in { let q: &dyn Query = g; created_0.clone() } {
let selected_6_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_6_candidates = { let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Aura))).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } for id in ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Aura))).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all } }; let selected_6_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_6 = { cantrip_mechanics::choose::select(g, selected_6_player, asker.source, Purpose::PutOntoBattlefield, selected_6_candidates, cantrip_mechanics::select::Selection::of(selected_6_count, &[]))? };
let objects_7 = ({ let q: &dyn Query = g; selected_6.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_7 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_7 { let controller = g.state().obj(o).owner; if let Some(host) = { let q: &dyn Query = g; Some(each_object) } { moved.extend(g.put_onto_battlefield_attached(o, controller, host)?); } } moved };
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Liberated Livestock", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Cat,Subtype::Bird,Subtype::Ox]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this creature dies, create a 1/1 white Cat creature token with lifelink, a 1/1 white Bird creature token with flying, and a 2/4 white Ox creature token. For each of those tokens, you may put an Aura card from your hand and/or graveyard onto the battlefield attached to it.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD,&token_2::CARD,&token_3::CARD];
