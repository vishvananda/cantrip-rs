// Peace Talks
// status: planned
// requires: continuous rule Prohibit BeTargeted
// requires: duration ThroughNextTurn
// requires: engine: no resolving rule on an open set: Prohibit Attack
// requires: engine: no resolving rule on an open set: Prohibit BeTargeted
// requires: object filter OnStack
// requires: player rule for a while Prohibit BeTargeted
// This turn and next turn, creatures can't attack, and players and permanents can't be the targets of spells or activated abilities.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("engine-no-resolving-rule-on-an-open-set-prohibit-attack" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { g.add_open_effect(source, move |q: &dyn Query, asker: &Asker, id: ObjectId| q.chars(id).is(CardType::Creature), EffectKind::CantAttack, planned_value!("duration ThroughNextTurn", Duration::Indefinite => Duration::ThroughNextTurn)); });
g.end_sentence();
planned!("player-rule-for-a-while-prohibit-be-targeted" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::Cant(Prohibition::BeTargeted(|q: &dyn Query, asker: &Asker, by: &Asker, by_source: &Chars| ((by.spell && true) || planned_value!("object filter OnStack", false => q.stack_item(by.source).is_some_and(|i| i.is(StackTrait::Activated))))))), planned_value!("duration ThroughNextTurn", Duration::Indefinite => Duration::ThroughNextTurn)); } });
g.end_sentence();
planned!("engine-no-resolving-rule-on-an-open-set-prohibit-be-targeted" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { g.add_open_effect(source, move |q: &dyn Query, asker: &Asker, id: ObjectId| true, planned_value!("continuous rule Prohibit BeTargeted", EffectKind::CantBlock => EffectKind::Rule(Rule::Cant(Action::BeTargeted(|q: &dyn Query, asker: &Asker, by: &Asker, by_source: &Chars| ((by.spell && true) || planned_value!("object filter OnStack", false => q.stack_item(by.source).is_some_and(|i| i.is(StackTrait::Activated)))))))), planned_value!("duration ThroughNextTurn", Duration::Indefinite => Duration::ThroughNextTurn)); });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Peace Talks", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
