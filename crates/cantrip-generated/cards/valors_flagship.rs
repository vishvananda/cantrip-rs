// Valor's Flagship
// status: ready
// Flying, first strike, lifelink
// Crew 3
// Cycling {X}{2}{W} ({X}{2}{W}, Discard this card: Draw a card.)
// When you cycle this card, create X 1/1 colorless Pilot creature tokens with "This token saddles Mounts and crews Vehicles as though its power were 2 greater."
#[allow(unused_imports)] use cantrip_api::*;
fn ability_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_5_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Cycled { card, player } = event { let id = *card; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && true && id == asker.source { return Some(Fired { subject, data }); } }
 None }
pub mod token_2 {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::CrewsAsThoughPowerTwoGreater);
chars.keywords = chars.keywords.with(Keyword::SaddlesAsThoughPowerTwoGreater); }

pub static CARD: CardDef = CardDef { name: token!(Pilot).name, token_words: token!(Pilot).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Pilot]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, cantrip_mechanics::statics::itself, ability_0_apply_0, "This token saddles Mounts and crews Vehicles as though its power were 2 greater.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_5(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_1 = Vec::new();
let count_1_0 = { let q: &dyn Query = g; (q.state().stack.iter().chain(q.state().left_stack.iter()).rev().find(|i| i.kind == StackKind::Activated && q.obj(i.object).became == Some(asker.source)).map_or(0i64, |i| i64::from(i.x))) as u32 }; let def_1_0 = g.card_id(&token_2::CARD);
for player in players_1 {
tokens_1.extend(std::iter::repeat((def_1_0,player)).take(count_1_0 as usize));
} let words_1 = g.words(asker.source);
let created_1 = g.create_tokens_worded(&tokens_1, false, None, words_1)?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Valor's Flagship", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [3, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Vehicle]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(7), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::FirstStrike,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::crew::crew(3, "Crew 3"),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 1, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(DISCARD_THIS), ability_4, "Cycling {X}{2}{W}").targets(&[]).cycling().from(ZoneKind::Hand),triggered(when_asked(EventMask(EventMask::CYCLED.0), ability_5_event).functions_in(WHERE_A_DISCARD_ENDS), ability_5, "When you cycle this card, create X 1/1 colorless Pilot creature tokens with \"This token saddles Mounts and crews Vehicles as though its power were 2 greater.\"").targets(&[]).functions_in(ZoneKinds::of(&[ZoneKind::Battlefield, ZoneKind::Graveyard, ZoneKind::Hand, ZoneKind::Library, ZoneKind::Stack, ZoneKind::Exile]))], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_2::CARD];
