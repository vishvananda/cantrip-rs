// Zurgo and Ojutai
// status: planned
// requires: object collection Damage
// Flying, haste
// Zurgo and Ojutai has hexproof as long as it entered this turn.
// Whenever one or more Dragons you control deal combat damage to a player or battle, look at the top three cards of your library. Put one of them into your hand and the rest on the bottom of your library in any order. You may return one of those Dragons to its owner's hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.source).is_some_and(|candidate: ObjectId| (q.history().entered().filter(|e| e.permanent == candidate).count() as i64) >= (1i64))) && (source == affected) }
#[allow(unused_variables)] fn ability_2_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Hexproof); }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && (q.chars(source).has_subtype(q.words(asker.source).subtype(Subtype::Dragon)) && Some(asker.controller) == Some(q.controller(source))) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Object(o) => o.0, _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && (q.chars(source).has_subtype(q.words(asker.source).subtype(Subtype::Dragon)) && Some(asker.controller) == Some(q.controller(source))) && matches!(*target, Target::Object(o) if q.chars(o).is(CardType::Battle)) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let depth_0 = { let q: &dyn Query = g; (3i64).max(0) as usize };  let taken_0 = cantrip_mechanics::library::look_at_top(g, player_0, asker.source, depth_0, (1, 1), false, |_| true, cantrip_mechanics::library::Rest::BottomChosen)?;
g.end_sentence();
{
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; planned_value!("object collection Damage", Vec::<ObjectId>::new() => q.damage_objects(item, DamageSubject::Source)) }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::PutIntoHand, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
if !selected_1.is_empty() { 
let objects_2 = { let q: &dyn Query = g; selected_1.clone() }; let moved_2 = g.return_to_hand(&objects_2);
g.end_sentence();
hoisted_3.extend(moved_2.clone());
hoisted_4.extend(selected_1.clone());
hoisted_5.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Zurgo and Ojutai", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Orc,Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_3_event), ability_3, "Whenever one or more Dragons you control deal combat damage to a player or battle, look at the top three cards of your library. Put one of them into your hand and the rest on the bottom of your library in any order. You may return one of those Dragons to its owner's hand.").targets(&[]).once_per_batch().batch_by_data()], statics: &[static_ability(Layer::Ability, ability_2_0_affects, ability_2_0_apply_0, "Zurgo and Ojutai has hexproof as long as it entered this turn.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
