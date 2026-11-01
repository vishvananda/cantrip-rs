// Winter Soldier, Icy Assassin
// status: ready
// Vigilance, menace
// Winter Soldier gets +2/+0 for each Equipment attached to him.
// {3}{W}{B}: Return this card from your graveyard to the battlefield with a finality counter on him. Then you may attach an Equipment you control to him. (If a creature with a finality counter on it would die, exile it instead.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = (((2i64 * (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Equipment)) && q.state().obj(id).attached_to.is_some_and(|host| Some(asker.source) == Some(host)))).collect::<Vec<ObjectId>>()).len() as i64)).clamp(-32768, 32767) as i16, ((0i64 * (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Equipment)) && q.state().obj(id).attached_to.is_some_and(|host| Some(asker.source) == Some(host)))).collect::<Vec<ObjectId>>()).len() as i64)) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_0 = ({ let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_0 = { let rows_0: Vec<(ObjectId, PlayerId, bool)> = objects_0.iter().map(|&o| (o, player_0, false)).collect(); let counters = [(CounterKind::Finality, { let q: &dyn Query = g; (1i64).max(0) as i16 })]; g.put_all_onto_battlefield_with_counters(&rows_0, &counters)? };
g.end_sentence();
{
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let eligible_2 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Equipment)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let host_2 = { let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied() }; let eligible_2 = if host_2.is_some() { eligible_2 } else { Vec::new() }; let count_2 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let chosen_2 = cantrip_mechanics::choose::select(g, player_2, asker.source, Purpose::Attach, eligible_2, cantrip_mechanics::select::Selection::of(count_2, &[]))?; if let Some(host) = host_2 { cantrip_api::instruction(g, |g| { for &attachment in &chosen_2 { g.attach(attachment, host); } }); }
if !chosen_2.is_empty() { 
g.end_sentence();
hoisted_3.extend(chosen_2.clone());
}
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Winter Soldier, Icy Assassin", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Assassin,Subtype::Villain]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance,Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [1, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_3, "{3}{W}{B}: Return this card from your graveyard to the battlefield with a finality counter on him. Then you may attach an Equipment you control to him.").targets(&[]).from(ZoneKind::Graveyard)], statics: &[static_ability(Layer::PtModify, cantrip_mechanics::statics::itself, ability_2_apply_0, "Winter Soldier gets +2/+0 for each Equipment attached to him.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
