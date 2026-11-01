// Teferi's Realm
// status: planned
// requires: supertype World
// At the beginning of each player's upkeep, that player chooses artifact, creature, land, or non-Aura enchantment. All nontoken permanents of that type phase out. (While they're phased out, they're treated as though they don't exist. Each one phases in before its controller untaps during their next untap step.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; item.subject.player().expect("a reference that always names a player") }; let chosen_0 = cantrip_mechanics::choose::option(g, player_0, asker.source, item.ability.card, 0..4)? - 0;
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (!(q.state().obj(id).token) && match chosen_0 { 0 => q.chars(id).is(CardType::Artifact), 1 => q.chars(id).is(CardType::Creature), 2 => q.chars(id).is(CardType::Land), _ => (!(q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Aura))) && q.chars(id).is(CardType::Enchantment)) })).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; g.phase_out(&objects_1);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Teferi's Realm", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: planned_value!("supertype World", Supertypes { basic: false, legendary: false, snow: false } => Supertypes { basic: false, legendary: false, snow: false, world: true }), colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of each player's upkeep, that player chooses artifact, creature, land, or non-Aura enchantment. All nontoken permanents of that type phase out.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["artifact", "creature", "land", "nonAura enchantment"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
