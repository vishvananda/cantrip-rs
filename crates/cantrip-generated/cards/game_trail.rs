// Game Trail
// status: ready
// As this land enters, you may reveal a Mountain or Forest card from your hand. If you don't, this land enters tapped.
// {T}: Add {R} or {G}.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let payer_0 = { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") }; let (paid_0, after_0, before_0, picked_0): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)? {
let selected_2_player = { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") }; let selected_2_candidates = { let q: &dyn Query = g; ((Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Mountain)) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Forest)))).collect::<Vec<ObjectId>>() }; let selected_2_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_2 = { cantrip_mechanics::choose::select(g, selected_2_player, asker.source, Purpose::Reveal, selected_2_candidates, cantrip_mechanics::select::Selection::of(selected_2_count, &[]))? };
let revealer_1 = { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") }; g.reveal(revealer_1, &selected_2); let paid_1 = !selected_2.is_empty();
(paid_1, Vec::new(), Vec::new(), { let q: &dyn Query = g; (selected_2).clone() }) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
if { let q: &dyn Query = g; !(((paid_0 as i64) != 0)) } {
entering.tapped = true;
}
 Ok(()) }
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_4 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_4 {
let color = chosen.expect("mana activation selected a color");
g.add_mana(player, color.mana(), mana_4);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Game Trail", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_1_mana).choose_color().colors(ColorSet(24)).yields_any(1), "{T}: Add {R} or {G}.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
