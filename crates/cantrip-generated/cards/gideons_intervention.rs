// Gideon's Intervention
// status: ready
// As this enchantment enters, choose a card name.
// Your opponents can't cast spells with the chosen name.
// Prevent all damage that would be dealt to you and permanents you control by sources with the chosen name.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let player_0 = entering.controller; let Answer::Name(Some(chosen_0)) = g.ask(player_0, RequestKind::ChooseName { source: entering.object, looked_at: Vec::new(), suggestion: None, kind: NameKind::ANY })? else { return Ok(()) };
entering.chosen_card = Some(chosen_0);
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !(player != asker.controller && q.state().chosen_card(q.linked_to(asker)).is_some_and(|chosen| cantrip_mechanics::library::object_has_name(q, spell, chosen))) }
#[allow(unused_variables)] fn ability_2_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && q.state().chosen_card(q.linked_to(asker)).is_some_and(|chosen| cantrip_mechanics::library::object_has_name(q, source, chosen)) && match *target { Target::Object(o) => Some(asker.controller) == Some(q.controller(o)), Target::Player(p) => Some(asker.controller) == Some(p), _ => false } { return true; } }
 false }

pub static CARD: CardDef = CardDef { name: "Gideon's Intervention", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[PreventionDef { hint: "prevents all damage that would be dealt to you and permanents you control by sources with the chosen name", applies: ability_2_applies, amount: None }], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, spell_rules: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
