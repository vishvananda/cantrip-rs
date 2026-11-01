// Akroma, Angel of Wrath Avatar
// status: planned
// requires: a choice of Ability outside its body
// requires: card type Vanguard
// requires: layout Vanguard
// requires: operation ChooseValueWith
// Whenever a creature you control enters, it gains two abilities chosen at random from among flying, first strike, trample, haste, protection from black, protection from red, and vigilance.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-choose-value-with" => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Ability, ChoiceTerms { options: &["Ability(Keyword(Flying))", "Ability(Keyword(FirstStrike))", "Ability(Keyword(Trample))", "Ability(Keyword(Haste))", "Ability(Keyword(Protection(Objects { kind: Any, filter: Color(Black) })))", "Ability(Keyword(Protection(Objects { kind: Any, filter: Color(Red) })))", "Ability(Keyword(Vigilance))"], except: &[], count: Some({ let q: &dyn Query = g; (2i64).max(0) as u32 }), secret: false, unchosen: false, random: true, ordered: false, up_to: false })?);
g.end_sentence();
planned!("a-choice-of-ability-outside-its-body" => cantrip_mechanics::effects::grant(g, item, { let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() }, &[Grant::ChosenAbility], Duration::Indefinite)?);
g.end_sentence();
 Ok(()) }
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: 1, life: 7 }));

pub static CARD: CardDef = CardDef { name: "Akroma, Angel of Wrath Avatar", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "Whenever a creature you control enters, it gains two abilities chosen at random from among flying, first strike, trample, haste, protection from black, protection from red, and vigilance.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
