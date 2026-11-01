// Manascape Refractor
// status: planned
// requires: the activated abilities of other permanents
// This artifact enters tapped.
// This artifact has all activated abilities of all lands on the battlefield.
// You may spend mana as though it were mana of any color to pay the activation costs of this artifact's abilities.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
 Ok(()) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let ids = q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>(); planned_value!("the activated abilities of other permanents", () => cantrip_mechanics::statics::grant_activated_abilities_of_permanents(q, &ids, chars)); }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, s: &Spending) -> bool { let player = s.player; player == asker.controller && s.purpose == SpendingFor::Activating && s.source.is_some_and(|id| Some(asker.source) == Some(id)) }

pub static CARD: CardDef = CardDef { name: "Manascape Refractor", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, cantrip_mechanics::statics::itself, ability_1_apply_0, "This artifact has all activated abilities of all lands on the battlefield.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, spend_any_color: &[ability_2_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
