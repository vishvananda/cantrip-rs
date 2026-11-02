// Electro, Assaulting Battery
// status: planned
// requires: player rule RetainMana
// Flying
// You don't lose unspent red mana as steps and phases end.
// Whenever you cast an instant or sorcery spell, add {R}.
// When Electro leaves the battlefield, you may pay {X}. When you do, he deals X damage to target player.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mana_players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_1_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_1 {
g.add_mana(player, Mana::R, mana_1_0);}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn reflexive_6_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static REFLEXIVE_6_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_6_target_0)];
fn reflexive_6(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_7 = { let q: &dyn Query = g; Some(asker.source) }; let amount_7 = { let q: &dyn Query = g; (i64::from(item.x)) as u16 }; let recipients_7 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_7 { let batch: Vec<(ObjectId, Target, u16)> = recipients_7.iter().map(|t| (source, *t, amount_7)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
'optional_2: {
let payer_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let x_4 = g.may_pay_x(payer_4, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?; let paid_4 = x_4.is_some(); let bound_4: i64 = i64::from(x_4.unwrap_or(0));
if !paid_4 { break 'optional_2; }
if paid_4 && true { g.reflexive_trigger_counted(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None, (bound_4).clamp(0, i64::from(u16::MAX)) as u16); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");
planned_item!("player-rule-retain-mana" => statics: player_rule(ability_1_0_applies, PlayerRule::RetainMana(ColorSet(8))));

pub static CARD: CardDef = CardDef { name: "Electro, Assaulting Battery", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Villain]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_2_event), ability_2, "Whenever you cast an instant or sorcery spell, add {R}.").targets(&[]),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_3_event), ability_3, "When Electro leaves the battlefield, you may pay {X}. When you do, he deals X damage to target player.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_6, "When you do, he deals X damage to target player.").targets(REFLEXIVE_6_TARGETS)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
