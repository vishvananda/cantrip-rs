// Verrak, Warped Sengir
// status: planned
// requires: trigger event PaysToActivate
// Flying, deathtouch, lifelink
// Whenever you activate an ability that isn't a mana ability, if life was paid to activate it, you may pay that much life again. If you do, copy that ability. You may choose new targets for the copy.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_0 = g.may_pay(payer_0, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { life: ({ let q: &dyn Query = g; i64::from(item.data[1]) }).clamp(0, i64::from(u32::MAX)) as u32, ..Cost::NONE }), &PAY_QUESTION)?;
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let copier_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let t = item.subject; g.copy_ability(t, copier_1, true)?;
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");
planned_item!("trigger-event-pays-to-activate" => matcher: #[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::AbilityActivated { source, ability, player, .. } = event { let id = *source; let def = q.activated_ability(*ability); let paid = q.paid_to_activate(event); let subject = cantrip_mechanics::triggers::activated_ability_on_stack(q, event).unwrap_or(Target::None); let data = [id.0, paid.life]; if !(def.is_some_and(|d| d.is_mana_ability())) && Some(asker.controller) == Some(*player) && paid.life > 0 { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Verrak, Warped Sengir", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Vampire]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Deathtouch,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event PaysToActivate", EventMask(0) => EventMask(EventMask::ABILITY_ACTIVATED.0)), ability_3_event), ability_3, "Whenever you activate an ability that isn't a mana ability, if life was paid to activate it, you may pay that much life again. If you do, copy that ability. You may choose new targets for the copy.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
