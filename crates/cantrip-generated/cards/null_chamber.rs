// Null Chamber
// status: planned
// requires: static rule Prohibit BePlayed
// requires: supertype World
// As this enchantment enters, you and an opponent each choose a card name other than a basic land card name.
// Spells with the chosen names can't be cast and lands with the chosen names can't be played.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let player_0 = entering.controller; let candidates_0 = { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let Some(chosen_0) = cantrip_mechanics::choose::player(g, player_0, entering.object, candidates_0, "Choose", 0)? else { return Ok(()) };
entering.chosen_player = Some(chosen_0);
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; { let mut all: Vec<PlayerId> = Vec::new(); for p in Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() { if !all.contains(&p) { all.push(p); } } for p in Some(chosen_0).into_iter().collect::<Vec<PlayerId>>() { if !all.contains(&p) { all.push(p); } } all } };
let mut each_chosen_2 = Vec::new();
let mut each_chosen_2_who: Vec<PlayerId> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let player_2 = entering.controller; let Answer::Name(Some(chosen_2)) = g.ask(player_2, RequestKind::ChooseName { source: entering.object, looked_at: Vec::new(), suggestion: None, kind: NameKind::OTHER_THAN_BASIC_LAND })? else { return Ok(()) };
entering.chosen_card = Some(chosen_2);
 each_chosen_2.push(chosen_2);
each_chosen_2_who.push(bound_0);
   } 
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !(true && each_chosen_2.iter().any(|&chosen| cantrip_mechanics::library::object_has_name(q, spell, chosen))) }
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Land) && each_chosen_2.iter().any(|&chosen| !(*so_far).name.is_empty() && q.cards().get(chosen).name == (*so_far).name)) && { let o = q.state().obj(affected); !o.zone.is(ZoneKind::Battlefield) && !o.token && !o.card_copy } }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { let source = asker.source; (q.chars(id).is(CardType::Land) && each_chosen_2.iter().any(|&chosen| cantrip_mechanics::library::object_has_name(q, id, chosen))) && { let o = q.state().obj(id); !o.zone.is(ZoneKind::Battlefield) && !o.token && !o.card_copy } }
planned_item!("static-rule-prohibit-be-played" => statics: object_rule(ability_2_0_rule, Rule::Cant(Action::BePlayed)));

pub static CARD: CardDef = CardDef { name: "Null Chamber", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: planned_value!("supertype World", Supertypes { basic: false, legendary: false, snow: false } => Supertypes { basic: false, legendary: false, snow: false, world: true }), colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, spell_rules: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
