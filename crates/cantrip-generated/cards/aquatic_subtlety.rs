// Aquatic Subtlety
// status: planned
// requires: another way beside a mechanic's only way
// requires: engine: no perpetual effects
// Draw two cards, then put two cards from your hand on the bottom of your library in any order. Blue creature cards in your hand perpetually gain "Evoke—Exile a blue card from your hand."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn perpetual_5_6(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).colors.has(q.words(asker.source).color(Color::Blue)) }
static PERPETUAL_5: CardDef = CardDef { keywords: Keywords::of(&[]), abilities: &[cantrip_mechanics::evoke::sacrifice_trigger("When Aquatic Subtlety enters, if its evoke cost was paid, its controller sacrifices it.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: None, ways: &[cantrip_mechanics::evoke::way(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(exile_from_hand(1, perpetual_5_6)), "Evoke")], ..CardDef::BLANK };
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((2i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::PutOnBottom, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
let objects_2 = { let q: &dyn Query = g; selected_1.clone() }; let moved_2 = { let order_2 = match objects_2.iter().all(|&o| g.state().obj(o).owner == asker.controller) { true => cantrip_mechanics::library::ask_library_order(g, asker.controller, asker.source, &objects_2, DestinationKind::LibraryBottom)?, false => objects_2.clone() }; g.put_into_library(&order_2, LibraryPlace::Bottom) };
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_4: Vec<ObjectId> = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).colors.has(q.words(asker.source).color(Color::Blue)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() };
for &object in &objects_4 { let kind_4_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Rules(&PERPETUAL_5) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_4_0); });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Aquatic Subtlety", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 3, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
