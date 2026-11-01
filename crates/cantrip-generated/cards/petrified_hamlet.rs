// Petrified Hamlet
// status: planned
// requires: a choice of CardName outside its body
// When this land enters, choose a land card name.
// Activated abilities of sources with the chosen name can't be activated unless they're mana abilities.
// Lands with the chosen name have "{T}: Add {C}."
// {T}: Add {C}.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let Answer::Name(Some(chosen_0)) = g.ask(player_0, RequestKind::ChooseName { source: asker.source, looked_at: Vec::new(), suggestion: None, kind: NameKind { types: TypeSet::of(&[CardType::Land]), ..NameKind::ANY } })? else { return Ok(()) };
planned!("a-choice-of-card-name-outside-its-body" => let by = { let q: &dyn Query = g; q.linked(item) }; g.remember_chosen(by, ChosenValue::Card(chosen_0)););
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] #[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, object: ObjectId, ability: &ActivatedDef) -> bool { let player = q.controller(object); !(true && (planned_value!("a choice of CardName outside its body", None::<CardId> => q.state().chosen_card(q.linked_to(asker))).is_some_and(|chosen| cantrip_mechanics::library::object_has_name(q, object, chosen))) && (!(ability.is_mana_ability()))) }
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Land) && planned_value!("a choice of CardName outside its body", None::<CardId> => q.state().chosen_card(q.linked_to(asker.source))).is_some_and(|chosen| !(*so_far).name.is_empty() && q.cards().get(chosen).name == (*so_far).name)) }
fn granted_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_2_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_2 {
g.add_mana(player, Mana::C, mana_2_0);}
 Ok(()) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let card = { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }; let _ = card; if let Some(slot) = chars.abilities.iter().position(|a| a.is_none()) { chars.abilities[slot] = Some(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0)); } }
fn ability_3_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_3_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_3 {
g.add_mana(player, Mana::C, mana_3_0);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Petrified Hamlet", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this land enters, choose a land card name.").targets(&[]),mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 0, 0, 1]), ..add_by(ability_3_mana) }, "{T}: Add {C}.")], statics: &[static_ability(Layer::Ability, ability_2_affects, ability_2_apply_0, "Lands with the chosen name have \"{T}: Add {C}.\"")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 0, 0, 1]), ..add_by(granted_1_mana) }, "{T}: Add {C}.")], enters: None, modal: None, enchant: None, activation_rules: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
