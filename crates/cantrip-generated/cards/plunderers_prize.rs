// Plunderer's Prize
// status: planned
// requires: engine: no perpetual effects
// Seek a nonland artifact card with mana value X or less and put it onto the battlefield. If a card with mana value less than X is put onto the battlefield this way, this spell perpetually gains "This spell costs {1} more to cast," then return this spell to its owner's hand.
#[allow(unused_imports)] use cantrip_api::*;
static PERPETUAL_5: CardDef = CardDef { keywords: Keywords::of(&[]), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: None, additional_cost: Some(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }))), ..CardDef::BLANK };
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut pool_0 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((!(q.chars(id).is(CardType::Land)) && q.chars(id).is(CardType::Artifact)) && (i64::from(q.mana_value(id))) <= (i64::from(item.x)))).collect::<Vec<ObjectId>>() }; let count_0 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_0: Vec<ObjectId> = Vec::new(); while picked_0.len() < count_0 && !pool_0.is_empty() { let at = g.choose_at_random(pool_0.len()); picked_0.push(pool_0.remove(at)); }
let objects_1 = { let q: &dyn Query = g; picked_0.clone() }; let moved_1 = g.return_to_hand(&objects_1);
g.end_sentence();
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_2 = ({ let q: &dyn Query = g; (moved_1.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_2 = { let rows_2: Vec<(ObjectId, PlayerId, bool)> = objects_2.iter().map(|&o| (o, player_2, false)).collect(); let came_2 = g.put_all_onto_battlefield_as(&rows_2)?; came_2.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
if { let q: &dyn Query = g; (((moved_2.clone()).into_iter().filter(|&id| (i64::from(q.mana_value(id))) < (i64::from(item.x))).collect::<Vec<ObjectId>>()).len() as i64) > (0i64) } {
planned!("engine-no-perpetual-effects" => let objects_4: Vec<ObjectId> = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
for &object in &objects_4 { let kind_4_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Rules(&PERPETUAL_5) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_4_0); });
g.end_sentence();
let objects_6 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_6 = g.return_to_hand(&objects_6);
g.end_sentence();
hoisted_7.extend(moved_6.clone());
hoisted_8.extend({ let q: &dyn Query = g; objects_6.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Plunderer's Prize", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
