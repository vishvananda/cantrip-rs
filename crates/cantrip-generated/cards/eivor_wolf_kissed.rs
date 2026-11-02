// Eivor, Wolf-Kissed
// status: ready
// Trample, haste
// Whenever Eivor deals combat damage to a player, you mill that many cards. You may put a Saga card and/or a land card from among them onto the battlefield.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (i64::from(item.data[0])).max(0) as u32 }; let mills_0: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_0)).collect(); let milled_0 = g.mill_together(&mills_0);
g.end_sentence();
{
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let pool_2 = { let q: &dyn Query = g; (milled_0.clone()).into_iter().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga)) || q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }; let mut picked_2: Vec<ObjectId> = Vec::new();
{ let open = { let q: &dyn Query = g; pool_2.iter().copied().filter(|id| !picked_2.contains(id)).filter(|&id| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga))).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_2, asker.source, Purpose::PutOntoBattlefield, open, 1, true)?; for o in one { if !picked_2.contains(&o) { picked_2.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_2.iter().copied().filter(|id| !picked_2.contains(id)).filter(|&id| q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_2, asker.source, Purpose::PutOntoBattlefield, open, 1, true)?; for o in one { if !picked_2.contains(&o) { picked_2.push(o); } } }
let objects_3 = ({ let q: &dyn Query = g; picked_2.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_3 = { let rows_3: Vec<(ObjectId, PlayerId, bool)> = objects_3.iter().map(|&o| (o, g.state().obj(o).owner, false)).collect(); let came_3 = g.put_all_onto_battlefield_as(&rows_3)?; came_3.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_4.extend(moved_3.clone());
hoisted_5.extend({ let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });

}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Eivor, Wolf-Kissed", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Assassin,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(7), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_2_event), ability_2, "Whenever Eivor deals combat damage to a player, you mill that many cards. You may put a Saga card and/or a land card from among them onto the battlefield.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
