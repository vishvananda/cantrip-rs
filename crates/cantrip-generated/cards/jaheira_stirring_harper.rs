// Jaheira, Stirring Harper
// status: planned
// requires: engine: no perpetual effects
// requires: engine: no specialize event
// Hexproof from artifacts and enchantments
// When this creature specializes, destroy up to one target artifact or enchantment. You get a one-time boon with "When you cast a creature spell, it perpetually gets +1/+0 and gains haste."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Enchantment)))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, 1)];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
#[allow(unused_variables)] fn watch_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && q.chars(id).is(CardType::Creature) { return Some(Fired { subject, data }); } }
 None }
fn watch_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("engine-no-perpetual-effects" => let objects_4: Vec<ObjectId> = { let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() };
let kind_4_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump((1i64) as i16, (0i64) as i16)) };
let kind_4_1 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddKeywords(Keywords::of(&[Keyword::Haste]))) };
for &object in &objects_4 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_4_0); cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_4_1); });
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let destroyed_0 = g.destroy_all(&objects_0);
g.end_sentence();
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::UntilEvent, source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("engine-no-specialize-event" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Specialized { permanent } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Jaheira, Stirring Harper", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Elf,Subtype::Druid]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("engine: no specialize event", EventMask(0) => EventMask(EventMask::SPECIALIZED.0)), ability_1_event), ability_1, "When this creature specializes, destroy up to one target artifact or enchantment. You get a one-time boon with \"When you cast a creature spell, it perpetually gets +1/+0 and gains haste.\"").targets(ABILITY_1_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), watch_2_event), watch_2, "When this creature specializes, destroy up to one target artifact or enchantment.").targets(&[])], enters: None, modal: None, enchant: None, hexproof_from: HexproofFrom::NONE.and(HexproofFrom::card_type(CardType::Artifact)).and(HexproofFrom::card_type(CardType::Enchantment)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
