// Vampire Nocturnus Avatar
// status: planned
// requires: a library's top card open to view
// requires: card type Vanguard
// requires: layout Vanguard
// Play with the top card of your library revealed.
// As long as the top card of your library is black, black creatures you control get +2/+1.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("a-library-s-top-card-open-to-view" => #[allow(unused_variables)] fn library_top_open_0(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller });
#[allow(unused_variables)] fn ability_1_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>()).get(0).copied().is_some_and(|candidate: ObjectId| q.chars(candidate).colors.has(q.words(asker.source).color(Color::Black)))) && (((*so_far).colors.has(q.words(asker.source).color(Color::Black)) && (*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected)))) }
#[allow(unused_variables)] fn ability_1_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((2i64) as i16, (1i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
planned_item!("a-library-s-top-card-open-to-view" => play_rules: PlayRule::LibraryTopRevealed(library_top_open_0));
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: -1, life: -2 }));

pub static CARD: CardDef = CardDef { name: "Vampire Nocturnus Avatar", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::PtModify, ability_1_0_affects, ability_1_0_apply_0, "As long as the top card of your library is black, black creatures you control get +2/+1.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
