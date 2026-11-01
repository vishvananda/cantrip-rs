// Vedalken Squirrel-Whacker
// status: planned
// requires: event change as it enters a power and toughness read then
// requires: numeric expression Dice
// requires: replacement of RollD6
// As this creature enters, roll a six-sided die twice. Its base power becomes the first result and its base toughness becomes the second result.
// If you would roll one or more six-sided dice, instead roll them and you may exchange one result with this creature's base power or base toughness.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let runs_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; for _ in 0..runs_0 {
let player_1 = { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") }; let value_1 = i64::from(g.roll_die(player_1, 6).result);
}
planned!("event-change-as-it-enters-a-power-and-toughness-read-then" => entering.base_power_toughness = Some(((planned_value!("numeric expression Dice", 0i64 => g.last_roll(asker.controller).dice[1])).clamp(-32768, 32767) as i16, (planned_value!("numeric expression Dice", 0i64 => g.last_roll(asker.controller).dice[2])).clamp(-32768, 32767) as i16)););
 Ok(()) }
planned_item!("replacement-of-roll-d6" => callbacks: #[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::DieRoll(p) = proposal { let amount = p.count; let subject = Target::Player(p.player); let data = [p.count, u32::MAX]; if Some(asker.controller) == Some(p.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::DieRoll(seen) = &seen { let p = seen; let amount = p.count; let subject = Target::Player(p.player); let data = [p.count, u32::MAX]; if let Proposal::DieRoll(p) = proposal { p.exchange = Some(DieExchangeOffer { decider: Some(asker.controller).expect("a reference that always names a player"), object: Some(asker.source), properties: &[DieExchange::BasePower, DieExchange::BaseToughness] }); } } }
);
planned_item!("replacement-of-roll-d6" => replacements: replacement(ReplacementKind::DieRoll, ability_1_applies, ability_1_apply, "If you would roll one or more six-sided dice, instead roll them and you may exchange one result with this creature's base power or base toughness."));

pub static CARD: CardDef = CardDef { name: "Vedalken Squirrel-Whacker", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Vedalken,Subtype::Guest]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
