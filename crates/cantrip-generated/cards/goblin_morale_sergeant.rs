// Goblin Morale Sergeant
// status: planned
// requires: engine: no perpetual effects
// requires: operation Conjure
// Haste
// Enlist
// Whenever this creature enlists a nontoken creature, you may conjure a duplicate of that creature into the top five cards of your library at random. The duplicate perpetually gets +1/+0 and gains haste.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::PaidToAttack { attacker, option: AttackOption::Enlist, tapped, .. } = event { let id = *attacker; let other = *tapped; let subject = Target::Object(id); let data = [other.0, u32::MAX]; if Some(asker.source) == Some(id) && (!(q.state().obj(other).token) && q.chars(other).is(CardType::Creature)) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let duplicated_1 = { let q: &dyn Query = g; Some(ObjectId(item.data[0])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>() };
let count_1: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_1: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_1.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, Conjured::DuplicateOf(&duplicated_1), count_1, ConjureTo::Library(ConjureSpot::AmongTop({ let q: &dyn Query = g; (5i64).max(0) as usize })))?)); }
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_3: Vec<ObjectId> = { let q: &dyn Query = g; (conjured_1.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
let kind_3_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump((1i64) as i16, (0i64) as i16)) };
let kind_3_1 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddKeywords(Keywords::of(&[Keyword::Haste]))) };
for &object in &objects_3 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_3_0); cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_3_1); });
g.end_sentence();
hoisted_4.extend(conjured_1.clone());
hoisted_5.extend(conjured_1.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Goblin Morale Sergeant", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Goblin,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::enlist::ENLIST,triggered(when_asked(EventMask(EventMask::PAID_TO_ATTACK.0), ability_2_event), ability_2, "Whenever this creature enlists a nontoken creature, you may conjure a duplicate of that creature into the top five cards of your library at random. The duplicate perpetually gets +1/+0 and gains haste.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
