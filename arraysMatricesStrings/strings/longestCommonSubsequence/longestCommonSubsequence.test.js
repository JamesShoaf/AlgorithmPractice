const {
  longestCommonSubsequence,
} = require('./longestCommonSubsequence');

const testArrays = [
  [
    'abcde',
    'ace',
    3,
  ],
  [
    'abc',
    'abc',
    3,
  ],
  [
    'abc',
    'def',
    0,
  ],
  [
    'catatata',
    'california',
    3,
  ],
  [
    'catatatata',
    'californiaaaaaaaa',
    6,
  ],
  [
    'elefino',
    'fenio',
    3,
  ],
  [
    'Loremipsumdolorsitamet,consecteturadipiscingelit.Sedcongueatlorematfaucibus.Fusceeuvenenatissem,necsagittiselit.Inodioerat,ornareinpurusquis,commodoconsecteturmauris.Etiamsemper,risusacbibendumimperdiet,semvelitpulvinareros,nonsollicitudinnequeelitatsapien.Donecnecinterdumlectus,necsemperodio.Mauriseuscelerisquelectus,velauctorneque.Orcivariusnatoquepenatibusetmagnisdisparturientmontes,nasceturridiculusmus.Suspendissemattisliberoutmassasuscipitpretium.Aeneancommodoleolectus,sedrutrumorcivolutpata.Suspendisseeleifendestsitamettortorsodales,velaliquamarcusollicitudin.Quisquerutrumpulvinarconvallis.Curabiturvitaeligulautmassavehiculasuscipitmaximusaclorem.Proinvulputatepurusnecmalesuadarutrum.Vestibulumbibendumligulaorci,convallisvestibulumloreminterduma.Aeneanloremnunc,accumsanegetarcuefficitur,convallisultriciesnisi.Sedefficituretnibhacaliquam.Sedduimetus,aliqueteterosquis,aliquettempusdui.Morbilaciniaurnaneceratornareviverra.Quisquerutrumestnulla,sitametvulputatequamblanditeget.Donecfe',
    'Loremipsumdolorsitamet,consecteturadipiscingelit.Sedcongueatlorematfaucibus.Fusceeuvenenatissem,necsagittiselit.Inodioerat,ornareinpurusquis,commodoconsecteturmauris.Etiamsemper,risusacbibendumimperdiet,semvelitpulvinareros,nonsollicitudinnequeelitatsapien.Donecnecinterdumlectus,necsemperodio.Mauriseuscelerisquelectus,velauctorneque.Orcivariusnatoquepenatibusetmagnisdisparturientmontes,nasceturridiculusmus.Suspendissemattisliberoutmassasuscipitpretium.Aeneancommodoleolectus,sedrutrumorcivolutpata.Suspendisseeleifendestsitamettortorsodales,velaliquamarcusollicitudin.Quisquerutrumpulvinarconvallis.Curabiturvitaeligulautmassavehiculasuscipitmaximusaclorem.Proinvulputatepurusnecmalesuadarutrum.Vestibulumbibendumligulaorci,convallisvestibulumloreminterduma.Aeneanloremnunc,accumsanegetarcuefficitur,convallisultriciesnisi.Sedefficituretnibhacaliquam.Sedduimetus,aliqueteterosquis,aliquettempusdui.Morbilaciniaurnaneceratornareviverra.Quisquerutrumestnulla,sitametvulputatequamblanditeget.Donecfe',
    1000,
  ],
];

describe('longestCommonSubsequence', () => {
  test('it should return an integer', () => {
    testArrays.forEach((testTuple) => {
      expect(Number.isInteger(longestCommonSubsequence(...testTuple))).toBe(true);
    });
  });

  test('it should return the length of the longest subsequence', () => {
    testArrays.forEach((testTuple) => {
      expect(longestCommonSubsequence(...testTuple)).toBe(testTuple[2]);
    });
  });
});
