// Arcee, Sharpshooter // Arcee, Acrobatic Coupe
// status: planned
// requires: engine: no LivingMetal
// requires: engine: no MoreThanMeetsTheEye
// requires: engine: no cost removes counters this way
// requires: numeric expression PaidCountersRemoved
// requires: the members a trigger keeps, read through its source
// Living metal (During your turn, this Vehicle is also a creature.)
// Whenever you cast a spell that targets one or more creatures or Vehicles you control, put that many +1/+1 counters on Arcee. Convert Arcee.
// More Than Meets the Eye {R}{W} (You may cast this card converted for {R}{W}.)
// First strike
// {1}, Remove one or more +1/+1 counters from Arcee: It deals that much damage to target creature. Convert Arcee.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_target_0)];
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let amount_0 = { let q: &dyn Query = g; (planned_value!("numeric expression PaidCountersRemoved", 0i64 => i64::from(q.counters_paid(item)))).min(u16::MAX.into()) as u16 }; let recipients_0 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }; if let Some(source) = source_0 { let batch: Vec<(ObjectId, Target, u16)> = recipients_0.iter().map(|t| (source, *t, amount_0)).collect(); g.damage_together(&batch); }
g.end_sentence();
if let Some(object) = ({ let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.transform_by(object, item)?; }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("engine-no-more-than-meets-the-eye" => ways: cantrip_mechanics::more_than_meets_the_eye::way(ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "More Than Meets the Eye {R}{W}"));

pub static CARD: CardDef = CardDef { name: "Arcee, Sharpshooter", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Robot]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::FirstStrike]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("engine: no cost removes counters this way", Cost::NONE => Cost { variable: Some(VariableCost::RemoveAtLeast { kind: Some(CounterKind::PlusOne), least: 1 }), ..Cost::NONE })), ability_2, "{1}, Remove one or more +1/+1 counters from Arcee: It deals that much damage to target creature. Convert Arcee.").targets(ABILITY_2_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && true && q.state().stack.iter().find(|i| i.kind == StackKind::Spell && i.object == id).is_some_and(|i| { let wanted: Vec<ObjectId> = q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).is(CardType::Creature) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Vehicle))) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>(); q.state().targets(i).iter().filter_map(|t| t.object()).any(|o| wanted.contains(&o)) }) { return Some(Fired { subject, data }); } }
 None }
planned_item!("engine-no-living-metal" => statics: cantrip_mechanics::living_metal::ON_YOUR_TURN);
planned_item!("the-members-a-trigger-keeps-read-through-its-source", "(kind:Triggered,args:(event:SpellTargeting(players:(kind:One,args:(kind:AbilityController)),spells:(kind:Any),targets:Some((kind:Battlefield,args:(kind:All,args:[(kind:Either,args:[(kind:CardType,args:Creature),(kind:Subtype,args:Vehicle)]),(kind:ControlledBy,args:(kind:One,args:(kind:AbilityController)))]))),count:None),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:PutCounters,args:(placements:[(recipients:(kind:Objects,args:(kind:One,args:(kind:Source))),kind:PowerToughness(1,1),amount:(kind:EventAmount),selection:None)])))),(kind:Effect,args:(result:(1),operation:(kind:Keyword,args:(actor:(kind:One,args:(kind:AbilityController)),action:Convert,object:Some((kind:Source)),other:None,amount:None,subtype:None))))])))");

pub static CARD: CardDef = CardDef { name: "Arcee, Acrobatic Coupe", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Vehicle]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: Some(ColorSet(9)), power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
