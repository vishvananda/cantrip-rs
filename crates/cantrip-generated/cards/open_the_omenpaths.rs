// Open the Omenpaths
// status: planned
// requires: combined mana choices in one production
// requires: mana spending predicate
// Choose one —
// • Add two mana of any one color and two mana of any other color. Spend this mana only to cast creature or enchantment spells.
// • Creatures you control get +1/+0 until end of turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(dead_code, unused_variables)] fn mana_use_0(q: &dyn Query, asker: &Asker, spending: &Spending) -> bool { spending.purpose == SpendingFor::Casting && spending.source.is_some_and(|spell| (q.chars(spell).is(CardType::Creature) || q.chars(spell).is(CardType::Enchantment))) }
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("combined-mana-choices-in-one-production" => for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { cantrip_mechanics::mana::add_production(g, player, Production::All(&[Production::Repeat({ let q: &dyn Query = g; (2i64).max(0) as u32 }, &Production::ChooseOne(&[Production::Amount(ManaColor::White, { let q: &dyn Query = g; (1i64).max(0) as u32 }), Production::Amount(ManaColor::Blue, { let q: &dyn Query = g; (1i64).max(0) as u32 }), Production::Amount(ManaColor::Black, { let q: &dyn Query = g; (1i64).max(0) as u32 }), Production::Amount(ManaColor::Red, { let q: &dyn Query = g; (1i64).max(0) as u32 }), Production::Amount(ManaColor::Green, { let q: &dyn Query = g; (1i64).max(0) as u32 })]), ManaChoiceTiming::Once), Production::Repeat({ let q: &dyn Query = g; (2i64).max(0) as u32 }, &Production::OtherColor, ManaChoiceTiming::Once)]), Some(planned_value!("mana spending predicate", ManaRestriction::ABILITIES => ManaRestriction::Rule(&ManaRule { spend: mana_use_0, spent: None }))), None)?; });
g.end_sentence();
 Ok(()) }
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::EndOfTurn;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::Pump((1i64) as i16, (0i64) as i16) };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1);  } }); }
g.end_sentence();
 Ok(()) }
static ABILITY_0_MODES: ModalDef = modes(&[mode(ability_0_mode_0, "Add two mana of any one color and two mana of any other color. Spend this mana only to cast creature or enchantment spells.").targets(&[]),mode(ability_0_mode_1, "Creatures you control get +1/+0 until end of turn.").targets(&[])]).choose(1, 1);

pub static CARD: CardDef = CardDef { name: "Open the Omenpaths", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: Some(SpellDef::MODAL), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: Some(&ABILITY_0_MODES), enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
