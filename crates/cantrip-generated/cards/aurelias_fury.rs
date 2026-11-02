// Aurelia's Fury
// status: planned
// requires: player rule for a while can't cast some spells
// Aurelia's Fury deals X damage divided as you choose among any number of targets. Tap each creature dealt damage this way. Players dealt damage this way can't cast noncreature spells this turn.
#[allow(unused_imports)] use cantrip_api::*;
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY.between(1, TargetSpec::COUNTED)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
cantrip_mechanics::damage_divided(g, item);
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && cantrip_mechanics::history::permanents_dealt_damage_by(q.history(), asker.source, 0).contains(&id))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_1: Vec<ObjectId> = objects_1.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_1);
g.end_sentence();
planned!("player-rule-for-a-while-can-t-cast-some-spells" => let players_2_0 = { let q: &dyn Query = g; { let mut hurt: Vec<PlayerId> = Vec::new(); for r in q.history().all() { if let Event::Damage { source: s, target: Target::Player(p), amount, .. } = r.event { if s == asker.source && amount > 0 && !hurt.contains(&p) { hurt.push(p); } } } hurt } }; if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for &player in &players_2_0 { g.add_player_effect(asker.controller, source, EffectKind::CantCast { player, spells: SpellSet::matching(move |q, spell| !(q.chars(spell).is(CardType::Creature))) }, Duration::EndOfTurn); } });
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Aurelia's Fury", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, divide: Some(Divide { total: TargetSpec::COUNTED, of_x: None }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
