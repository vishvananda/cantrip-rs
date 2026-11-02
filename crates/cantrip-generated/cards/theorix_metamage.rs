// Theorix Metamage // Omit Variables
// status: ready
// This creature enters prepared. (While it's prepared, you may cast a copy of its spell. Doing so unprepares it.)
// Threshold — This creature gets +1/+0 and has flying as long as there are seven or more cards in your graveyard.
// Mill three cards. (Put the top three cards of your library into your graveyard.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.prepared = true;
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64) >= (7i64)) && (source == affected) }
#[allow(unused_variables)] fn ability_1_0_apply_1(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Flying); }
#[allow(unused_variables)] fn ability_1_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((1i64) as i16, (0i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }

pub static CARD: CardDef = CardDef { name: "Theorix Metamage", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::U, Mana::B)),None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Shade,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_1_0_affects, ability_1_0_apply_1, "Threshold — This creature gets +1/+0 and has flying as long as there are seven or more cards in your graveyard."),static_ability(Layer::PtModify, ability_1_0_affects, ability_1_0_apply_0, "Threshold — This creature gets +1/+0 and has flying as long as there are seven or more cards in your graveyard.").continues()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, prepare: Some(&prepared::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&prepared::CARD];
pub mod prepared {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let mills_0: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_0)).collect(); let milled_0 = g.mill_together(&mills_0);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Omit Variables", token_words: &[], oracle: false, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::U, Mana::B)),None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
