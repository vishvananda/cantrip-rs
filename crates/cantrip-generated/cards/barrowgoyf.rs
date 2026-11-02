// Barrowgoyf
// status: ready
// Deathtouch, lifelink
// Barrowgoyf's power is equal to the number of card types among cards in all graveyards and its toughness is equal to that number plus 1.
// Whenever this creature deals combat damage to a player, you may mill that many cards. If you do, you may put a creature card from among them into your hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = (({ let mut seen = TypeSet::NONE; for id in ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() { for kind in q.chars(id).types.iter() { seen = seen.with(kind); } } seen.iter().count() as i64 }).clamp(-32768, 32767) as i16, (({ let mut seen = TypeSet::NONE; for id in ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() { for kind in q.chars(id).types.iter() { seen = seen.with(kind); } } seen.iter().count() as i64 } + 1i64)).clamp(-32768, 32767) as i16); chars.power = Some(p); chars.toughness = Some(t); }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_0, after_0, before_0, picked_0): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)? {
let amount_2 = { let q: &dyn Query = g; (i64::from(item.data[0])) as u32 }; let mills_2: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_2)).collect(); let milled_2 = g.mill_together(&mills_2);
let paid_1 = true;
(paid_1, { let q: &dyn Query = g; (milled_2).clone() }, { let q: &dyn Query = g; (milled_2.clone()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
{
let selected_3_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_3_candidates = { let q: &dyn Query = g; (after_0.clone()).into_iter().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let selected_3_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_3 = { cantrip_mechanics::choose::select(g, selected_3_player, asker.source, Purpose::PutIntoHand, selected_3_candidates, cantrip_mechanics::select::Selection::of(selected_3_count, &[]))? };
if !selected_3.is_empty() { 
let objects_4 = { let q: &dyn Query = g; selected_3.clone() }; let moved_4 = g.return_to_hand(&objects_4);
g.end_sentence();
hoisted_5.extend(moved_4.clone());
hoisted_6.extend(selected_3.clone());
hoisted_7.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
hoisted_8.extend(hoisted_5.clone());
hoisted_9.extend(hoisted_6.clone());
hoisted_10.extend({ let q: &dyn Query = g; hoisted_7.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Barrowgoyf", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Lhurgoyf]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Deathtouch,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_3_event), ability_3, "Whenever this creature deals combat damage to a player, you may mill that many cards. If you do, you may put a creature card from among them into your hand.").targets(&[])], statics: &[characteristic_defining(Layer::PtDefine, ability_2_apply_0, "Barrowgoyf's power is equal to the number of card types among cards in all graveyards and its toughness is equal to that number plus 1.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
