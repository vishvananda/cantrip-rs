// Feather, Radiant Arbiter
// status: planned
// requires: a copy of a spell for each object it could target
// requires: object filter TargetableBy
// Flying, lifelink
// Whenever you cast a noncreature spell that targets only Feather, you may choose any number of other creatures that spell could target and pay {2} for each of those creatures. If you do, for each of those creatures, copy that spell. The copy targets that creature. (Copies of permanent spells become tokens.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (!(q.chars(id).is(CardType::Creature)) && (q.state().stack.iter().find(|i| i.object == id).is_some_and(|i| q.state().targets(i).len() == 1) && q.state().stack.iter().find(|i| i.kind == StackKind::Spell && i.object == id).is_some_and(|i| q.state().targets(i).iter().any(|t| t.object().is_some_and(|o| (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).contains(&o)))))) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_0 = cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)?; if paid_0 {
let payer_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let among_1: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Creature) && planned_value!("object filter TargetableBy", false => item.subject.object().is_some_and(|spell: ObjectId| q.targetable_by(spell, id))))).collect::<Vec<ObjectId>>() }; let picked_1 = cantrip_mechanics::choose::select(g, payer_1, asker.source, Purpose::Untap, among_1, cantrip_mechanics::select::cards(cantrip_mechanics::select::ANY_NUMBER))?; let price_1 = (Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }))).mana.times(picked_1.len() as u32); let chosen_1: Vec<ObjectId> = match price_1 { Some(mana) if !picked_1.is_empty() && g.may_pay(payer_1, asker.source, asker.source, Cost::mana(mana), &PAY_QUESTION)? => picked_1, _ => Vec::new() };
g.end_sentence();
hoisted_2.extend(chosen_1.clone());
hoisted_3.extend(chosen_1.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
planned!("a-copy-of-a-spell-for-each-object-it-could-target" => if let Some(spell) = { let q: &dyn Query = g; item.subject.object() } { cantrip_mechanics::copy::on_stack(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, spell, { let q: &dyn Query = g; (1i64).max(0) as u32 }, NewTargets::Each({ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (hoisted_3.clone()).contains(&id)).collect::<Vec<ObjectId>>() }))?; });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Feather, Radiant Arbiter", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [2, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_2_event), ability_2, "Whenever you cast a noncreature spell that targets only Feather, you may choose any number of other creatures that spell could target and pay {2} for each of those creatures. If you do, for each of those creatures, copy that spell. The copy targets that creature.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
