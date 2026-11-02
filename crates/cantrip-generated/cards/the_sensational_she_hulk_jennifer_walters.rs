// The Sensational She-Hulk // Jennifer Walters
// status: ready
// Reach, trample
// Your opponents can't cast spells during your turn.
// Whenever a creature you control is dealt damage, you may have The Sensational She-Hulk deal that much damage to any target. Do this only once each turn.
// Your opponents can't cast spells during your turn.
// {3}{G}{W}{W}: Transform Jennifer Walters. Activate only as a sorcery.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !((Some(asker.controller) == Some(q.state().turn.active)) && player != asker.controller && true) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(object) = ({ let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.transform_by(object, item)?; }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Jennifer Walters", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Advisor,Subtype::Hero]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [2, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_1, "{3}{G}{W}{W}: Transform Jennifer Walters. Activate only as a sorcery.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, spell_rules: &[ability_0_0_0_rule], back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !((Some(asker.controller) == Some(q.state().turn.active)) && player != asker.controller && true) }
static ABILITY_3_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = *target; let data = [u32::from(amount), match target { Target::Object(o) => o.0, _ => u32::MAX }]; if true && true && matches!(*target, Target::Object(o) if (q.chars(o).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(o)))) && !q.history().this_turn().iter().any(|r| matches!(r.event, Event::Done { ability, player, .. } if ability == AbilityRef { card: q.chars(asker.source).card, index: 200 } && player == asker.controller)) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let decider_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if !g.history().done(asker.source, AbilityRef { card: g.chars(asker.source).card, index: 200 }, decider_1) && cantrip_mechanics::effects::ask_may(g, decider_1, asker.source, Purpose::Optional)? { g.note_done(asker.source, AbilityRef { card: g.chars(asker.source).card, index: 200 }, decider_1);
let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let amount_2 = { let q: &dyn Query = g; (i64::from(item.data[0])).clamp(0, 65535) as u16 }; let recipients_2 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_2 { let batch: Vec<(ObjectId, Target, u16)> = recipients_2.iter().map(|t| (source, *t, amount_2)).collect(); g.damage_together(&batch); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "The Sensational She-Hulk", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [2, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Gamma,Subtype::Hero]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Reach,Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_3_event), ability_3, "Whenever a creature you control is dealt damage, you may have The Sensational She-Hulk deal that much damage to any target. Do this only once each turn.").targets(ABILITY_3_TARGETS).sums_per_subject()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, spell_rules: &[ability_2_0_0_rule], ..CardDef::BLANK };
}
