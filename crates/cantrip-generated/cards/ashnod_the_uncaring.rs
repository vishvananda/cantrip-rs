// Ashnod the Uncaring
// status: planned
// requires: trigger event PaysToActivate
// Deathtouch
// Whenever you activate an ability of an artifact or creature that isn't a mana ability, if one or more permanents were sacrificed to activate it, you may copy that ability. You may choose new targets for the copy. (Sacrificing an artifact for mana to activate an ability doesn't count.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let copier_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let t = item.subject; g.copy_ability(t, copier_0, true)?;
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-pays-to-activate" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::AbilityActivated { source, ability, player, .. } = event { let id = *source; let def = q.activated_ability(*ability); let paid = q.paid_to_activate(event); let subject = cantrip_mechanics::triggers::activated_ability_on_stack(q, event).unwrap_or(Target::None); let data = [id.0, paid.life]; if ((def.is_some_and(|d| d.from == ZoneKind::Battlefield) && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature))) && !(def.is_some_and(|d| d.is_mana_ability()))) && Some(asker.controller) == Some(*player) && !paid.sacrificed.is_empty() { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Ashnod the Uncaring", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Artificer]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Deathtouch]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event PaysToActivate", EventMask(0) => EventMask(EventMask::ABILITY_ACTIVATED.0)), ability_1_event), ability_1, "Whenever you activate an ability of an artifact or creature that isn't a mana ability, if one or more permanents were sacrificed to activate it, you may copy that ability. You may choose new targets for the copy.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
