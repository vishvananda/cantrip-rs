// Vikya, Scorching Stalwart
// status: planned
// requires: engine: no untap symbol cost
// Training (Whenever this creature attacks with another creature with greater power, put a +1/+1 counter on this creature.)
// {4}{R}, {Q}, Discard a card: Vikya, Scorching Stalwart deals damage equal to its power to any target. If excess damage was dealt to a creature this way, draw a card. ({Q} is the untap symbol.)
#[allow(unused_imports)] use cantrip_api::*;
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut done_0: i64 = 0; let mut excess_0: Vec<(Target, u32)> = Vec::new(); let since_0 = g.history().head() as usize; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let amount_0 = { let q: &dyn Query = g; (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))).max(0) as u16 }; let recipients_0 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_0 { let batch: Vec<(ObjectId, Target, u16)> = recipients_0.iter().map(|t| (source, *t, amount_0)).collect(); done_0 += g.damage_together(&batch).iter().map(|&n| i64::from(n)).sum::<i64>(); excess_0.extend(g.history().all()[since_0..].iter().filter_map(|r| match r.event { Event::Damage { source: s, target, recipient_excess, .. } if s == source => Some((target, recipient_excess)), _ => None })); }
g.end_sentence();
if { let q: &dyn Query = g; (excess_0.iter().map(|(_, n)| i64::from(*n)).sum::<i64>()) > (0i64) } {
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Vikya, Scorching Stalwart", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::training::TRIGGER,ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("engine: no untap symbol cost", Cost::NONE => Cost { untap: true, ..Cost::NONE })).and(discard_cards(1)), ability_1, "{4}{R}, {Q}, Discard a card: Vikya, Scorching Stalwart deals damage equal to its power to any target. If excess damage was dealt to a creature this way, draw a card.").targets(ABILITY_1_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
