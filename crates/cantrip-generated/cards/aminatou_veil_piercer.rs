// Aminatou, Veil Piercer
// status: ready
// At the beginning of your upkeep, surveil 2. (Look at the top two cards of your library, then put any number of them into your graveyard and the rest on top of your library in any order.)
// Each enchantment card in your hand has miracle. Its miracle cost is equal to its mana cost reduced by {4}. (You may cast a card for its miracle cost when you draw it if it's the first card you drew this turn.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as usize }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::surveil(g, player_0, asker.source, amount_0)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_miracle(q: &dyn Query, asker: &Asker, card: ObjectId) -> bool { (q.state().obj(card).became.is_none() && q.state().obj(card).zone.kind() == ZoneKind::Hand && Some(asker.controller) == Some(q.state().obj(card).owner)) && q.chars(card).is(CardType::Enchantment) }
fn ability_1_miracle_cost(q: &dyn Query, card: CardId) -> Option<ManaCost> { let def = q.card(card); def.has_cost.then_some(def.cost) }

pub static CARD: CardDef = CardDef { name: "Aminatou, Veil Piercer", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your upkeep, surveil 2.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_miracles: &[GrantedMiracle { text: "Aminatou, Veil Piercer's miracle: its mana cost reduced by {4}", has: ability_1_miracle, cost: ability_1_miracle_cost, less: 4 }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
