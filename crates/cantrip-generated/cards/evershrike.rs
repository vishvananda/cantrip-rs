// Evershrike
// status: planned
// requires: several objects arriving attached
// Flying
// This creature gets +2/+2 for each Aura attached to it.
// {X}{W/B}{W/B}: Return this card from your graveyard to the battlefield. You may put an Aura card with mana value X or less from your hand onto the battlefield attached to it. If you don't, exile this creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = (((2i64 * (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Aura)) && q.state().obj(id).attached_to.is_some_and(|host| Some(asker.source) == Some(host)))).collect::<Vec<ObjectId>>()).len() as i64)).clamp(-32768, 32767) as i16, ((2i64 * (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Aura)) && q.state().obj(id).attached_to.is_some_and(|host| Some(asker.source) == Some(host)))).collect::<Vec<ObjectId>>()).len() as i64)).clamp(-32768, 32767) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_0 = ({ let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_0 = { let rows_0: Vec<(ObjectId, PlayerId, bool)> = objects_0.iter().map(|&o| (o, player_0, false)).collect(); let came_0 = g.put_all_onto_battlefield_as(&rows_0)?; came_0.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
let payer_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_1, after_1, before_1, picked_1): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let selected_3_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_3_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Aura)) && (i64::from(q.mana_value(id))) <= (i64::from(item.x)))).collect::<Vec<ObjectId>>() }; let selected_3_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_3 = { cantrip_mechanics::choose::select(g, selected_3_player, asker.source, Purpose::PutOntoBattlefield, selected_3_candidates, cantrip_mechanics::select::Selection::of(selected_3_count, &[]))? };
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_2 = ({ let q: &dyn Query = g; selected_3.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_2 = planned_value!("several objects arriving attached", Vec::<ObjectId>::new() => { let rows_2: Vec<(ObjectId, PlayerId)> = objects_2.iter().map(|&o| (o, player_2)).collect(); g.put_all_onto_battlefield_attached(&rows_2, { let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied() })? });
let paid_2 = !moved_2.clone().is_empty();
(paid_2, { let q: &dyn Query = g; (moved_2).clone() }, { let q: &dyn Query = g; (objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).clone() }, { let q: &dyn Query = g; (selected_3).clone() }) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; !(((paid_1 as i64) != 0)) } {
let objects_4 = { let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let moved_4 = g.exile_all(&objects_4);
g.end_sentence();
hoisted_5.extend(moved_4.clone());
hoisted_6.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Evershrike", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::W, Mana::B)),Some((Mana::W, Mana::B)),None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental,Subtype::Spirit]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::W, Mana::B)),Some((Mana::W, Mana::B)),None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_2, "{X}{W/B}{W/B}: Return this card from your graveyard to the battlefield. You may put an Aura card with mana value X or less from your hand onto the battlefield attached to it. If you don't, exile this creature.").targets(&[]).from(ZoneKind::Graveyard)], statics: &[static_ability(Layer::PtModify, cantrip_mechanics::statics::itself, ability_1_apply_0, "This creature gets +2/+2 for each Aura attached to it.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
