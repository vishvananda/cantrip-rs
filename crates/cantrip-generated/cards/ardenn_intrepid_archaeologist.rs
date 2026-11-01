// Ardenn, Intrepid Archaeologist
// status: planned
// requires: operation AttachEach
// At the beginning of combat on your turn, you may attach any number of Auras and Equipment you control to target permanent or player.
// Partner (You can have two commanders if both have partner.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { match t { Target::Player(p) => true, Target::Object(id) => q.is_on_battlefield(id) && { let c = q.chars(id); c.is(CardType::Creature) || c.is(CardType::Planeswalker) || c.is(CardType::Battle) } && (true), _ => false } }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
planned!("operation-attach-each" => cantrip_mechanics::attach::each(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Aura)) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Equipment))) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }, { let q: &dyn Query = g; t_0.clone() }, Some({ let q: &dyn Query = g; cantrip_mechanics::select::Count::AnyNumber }), false)?);
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Ardenn, Intrepid Archaeologist", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Kor,Subtype::Scout]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of combat on your turn, you may attach any number of Auras and Equipment you control to target permanent or player.").targets(ABILITY_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
