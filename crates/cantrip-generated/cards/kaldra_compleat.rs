// Kaldra Compleat
// status: ready
// Living weapon
// Indestructible
// Equipped creature gets +5/+5 and has first strike, trample, indestructible, haste, and "Whenever this creature deals combat damage to a creature, exile that creature."
// Equip {7}
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn granted_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Object(o) => o.0, _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Object(o) if q.chars(o).is(CardType::Creature)) { return Some(Fired { subject, data }); } }
 None }
fn granted_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; Some(ObjectId(item.data[1])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>() }; let moved_1 = g.exile_all(&objects_1);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_apply_1(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::FirstStrike);
chars.keywords = chars.keywords.with(Keyword::Trample);
chars.keywords = chars.keywords.with(Keyword::Indestructible);
chars.keywords = chars.keywords.with(Keyword::Haste);
let card = { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }; let _ = card; if let Some(slot) = chars.abilities.iter().position(|a| a.is_none()) { chars.abilities[slot] = Some(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0)); } }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((5i64) as i16, (5i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
#[allow(unused_variables)] fn ability_3_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_3_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_3_target_0)];
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let attachments_2 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let host_2 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; if let Some(host) = host_2 { cantrip_api::instruction(g, |g| { for attachment in attachments_2 { g.attach(attachment, host); } }); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Kaldra Compleat", token_words: &[], oracle: true, cost: ManaCost { generic: 7, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Equipment]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Indestructible]), protection: ColorSet(0), spell: None, abilities: &[triggered(cantrip_mechanics::triggers::THIS_ENTERS, cantrip_mechanics::attach::living_weapon, "Living weapon"),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 7, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_3, "Equip {7}").targets(ABILITY_3_TARGETS).equip().sorcery_speed()], statics: &[static_ability(Layer::Ability, cantrip_mechanics::attach::attached_object, ability_2_apply_1, "Equipped creature gets +5/+5 and has first strike, trample, indestructible, haste, and \"Whenever this creature deals combat damage to a creature, exile that creature.\""),static_ability(Layer::PtModify, cantrip_mechanics::attach::attached_object, ability_2_apply_0, "Equipped creature gets +5/+5 and has first strike, trample, indestructible, haste, and \"Whenever this creature deals combat damage to a creature, exile that creature.\"").continues()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), granted_0_event), granted_0, "Whenever this creature deals combat damage to a creature, exile that creature.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&cantrip_mechanics::tokens::GERM];
