// Angel of Salvation
// status: ready
// Flash; convoke (Your creatures can help cast this spell. Each creature you tap while casting this spell pays for {1} or one mana of that creature's color.)
// Flying
// When this creature enters, prevent the next 5 damage that would be dealt this turn to any number of targets, divided as you choose.
#[allow(unused_imports)] use cantrip_api::*;
static ABILITY_3_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY.between(1, cantrip_api::TargetSpec::ANY_NUMBER)];
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn made_1_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && true { return true; } }
 false }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
for slot in 0..item.target_count as usize { let to = g.state().target(item, slot); let shield = u16::try_from(g.state().share(item, slot)).unwrap_or(u16::MAX); if shield > 0 && g.target_legal(item, slot) { let about = to.object().unwrap_or(item.object); g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::Next(shield) }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Angel of Salvation", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash,Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_3_event), ability_3, "When this creature enters, prevent the next 5 damage that would be dealt this turn to any number of targets, divided as you choose.").targets(ABILITY_3_TARGETS).divide(Divide { total: 5, of_x: None })], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "When this creature enters, prevent the next 5 damage that would be dealt this turn to any number of targets, divided as you choose.", damage: "damage", of_it: None, applies: made_1_applies, then: None, text: "When this creature enters, prevent the next 5 damage that would be dealt this turn to any number of targets, divided as you choose." }], pay_with: PayWith { convoke: true, ..PayWith::NONE }, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
