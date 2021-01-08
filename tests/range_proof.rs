use rand_core::SeedableRng;

use rand_chacha::ChaChaRng;

use curve25519_dalek::constants::RISTRETTO_BASEPOINT_TABLE;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;

use merlin::Transcript;

use tari_bulletproofs::{
    range_proof::{get_rewind_nonce_from_pub_key, get_secret_nonce_from_pvt_key},
    BulletproofGens, PedersenGens, ProofError, RangeProof,
};

use hex;
use rand::thread_rng;

// Tests that proofs generated with v1.0.0 continue to verify in later versions.
#[test]
fn deserialize_and_verify() {
    // proofs[i][j] has bitsize n = 8 << i, aggregation size m = 1 << j
    let proofs = [
        [
            b"46b6ea8b6a9710c41c2622d4b353dbcf5f89afe8ed66c469f192bec19dc71d23c0442827f97fc9085a89caa87d294b0a21e7b8957732ec4951f6bf7d3aa2c66e7af3b7b956c7dcb3bed1223575a217a30642b603b6bf1d4138ed95e3458c524510b42c8d82958f40b447a84242b1ba1eeea54013f80bad643048eeb0b17c292a057cb6ae1c42338837c05eaa6336a17d60fa141204e015a1df15b28c1318c709d7eb35569cde89c0bf37eace54880a151498b38da54c6d739564f46f01b73601e518355ea06c9ef58a45fcb3baadbd1ac54e0838c471a6b91845f123d569fa0c46ef94471b7b826230e8576146beec08ac3e6683998815c576581f4c0e493433480f95f6495210636eaa2e32b577e1c363e35e522db85b18a56d57eb626f9e2b50578e0d7ee7b74b328e158b366bb9d117db725820a2fec3b1508212d75823345a801c0b602bfa05919d7e3bb8e71944587072badc363f334b08ba90d13e077ad24b82bacd51fc668d2b880daabd3b87e6bdc9584af66523026a30aadfc359283891bb65cca502f47421ffeee1fb5a5237bfa965b66a8b8ca5d6954f4f8222244c6a5340dc81e8d781d092cae2a763f185dd0b89965b1dd2506807b5d3e5a305fd9a68e60b91389dcffae6f85538713aa7ed272b8174e2f0b9730ebb6c464d06".to_vec(),
            b"96cb7f843c95f494b5b97d74d29ad855eda25a9b09a51c04275d5f79b988055dee65c2accc2d77f281ff9e1933a470f40d467ae7a557e922a6d0cf07db09e3655a7d82cfe9262b7ad49953b765f0e85ff71730526d43b902b9e50e88f1b13a1ca0fed50530b4a22307a4eebf2d49127f1899a5b6b0b68f25d078972ec53a555fa371422ef112b34b08d45c37cca5bd6039f7644162e073918502a0585f9e3803e59f6e6b1646e38716a0704b8232f397c9ee6222a971dec8b8518b5dc6101308e188f22a47baf5139d069183a657287cabfe744e7f4871676d5b85321374710fd4f69cd96053c09ee529fe1e235765d3410593c7b8caff21c13d8efb175f540306bf4a841d05a2f78dae3e2382d4d0662f15f282b47a435a2b1d3f9bcaff0d14a4d44ba466ea0e63664656e715d60b42866626cd3a039d08c6428b3c4f60106610d97ead55e1af253353ddb313c65b4a6dd65ec4555c62dde57653db51db111ef42cfb0bf7621e14509634d3616ec9b46a12ab2b187668a6207a4838d04e0b01aca5f4002ee00a901319d876ab8ebb6619c81f2afc2016e48b7e1cf14577af1abccea0100f3ee10cbc1b2ea5f46e0a4a25519f5c58ef15c2c38549e9240a615fc87210c68e6128476cb02dd5ce2b8fdbd407f23013ed1ab3fa86ff441788b31ab5375e5442e825ecf5d7f3060e3b2bbf96074f5dc121ec6f3460d8d27f2e4101f5d910181c6012f6345da390055929f6c6ba243b6f8ef2a1a62c49e9ae036703".to_vec(),
            b"ac9cc69473001eb3a3c794b293a7ebfb5677441aa1050bbefdb700cbd1e33b42422b547213f86fb2321c51dc0433fcc3114d406ab4c6df6fbdd854d878710f5e6c8a02a6231ac47d4c7bb01311fd98563285c7accdb63a01037bea326fa269094609d0552446cc593a141f1b735704009ec08eba147beff77c61c13d969ba443240e8c341e25ae508a1c6394d804baa70902ec15bd556a031a6470f7fba40f0842a77f1a070d5bd6ad0b4fb336924bc79b863eeb62baaa06684b4e78ab806a05d8372ee5951478fc673a8d6291727ee3af2154dad109cb4258b15ead6e34360840675442fd7d6fea8a54a787247244a1749a193a67592dd14abff0ad9ee9fd66f6e9762b69fda1e2dd59c786c5ed40e12be542b34a0a70a4a372e33eb6def048f82fc612fffb186f910e7436aee4c9a37065c1298cd20f5025b16273310e69259adda0d53703fafba056e5ad45f854bd10d904f38f5265794f7241fcaa77171e2ad8ac27c85a5c25439c3af100715d6626744b01064a4c69df0875ba25cb6418dc3e2f02a2c91bef7783dd10e898fc132957c0d50fad4095e16d136142bbd24886e9642297f93302cf1c13c4eb6e593f35896c3c52c208899901c654190f2274a8254bf2e3a412e20dabdd9cce8fa4e2cd9ff6c850c7513cb70dc8eebfddd222509717707674b2dc477adcdb1dd68b3a05a43db5f39a1cc41269210098eb8473089441253de2ddfcfbf6451300f3848e13cc4c09729780bddc524015f80aa171133dd05a64d53dc2d752d6b5892980cd772eb78c991a66222e7489f83dcb4c0d3d7951528ae33427d117232b8571641769d190e941a47b2fabafa74127ede40b".to_vec(),
            b"debf1c3132cca627b5a254233a346397121c8316c24c36ad56c112faa556ba5b9e70c27ba6ebddd16fbade34093139cc2d110b92b07156e09afca4232aad8a53cc652db53258d4fc9470424162265ab473807807d563da8a312e360876948952b2b35314dafb99daaa5849eb1b2d4f6b9352c6c4d2c5bcf9486b7f741807b43bd3080f3cec4276bfe51be3c7a4830de67fab37ac05d9bf39211634afdb4aee00ae347d685f787ddd4fcdae659594a96e0b369350c048d323687108a87f4d800dc81635453e102239fc79f38e54163def5cb45b9654574450f663a86a23277d0d4e4e0a39541ae31d9ac9cc81b6f2152b1fa454f4cb7922e38d8a3f53449f9759ec0dbc37426515968f71b17f0737ccc0162fe9462f9ee3c9e060894ec7a4527faa58949f663d79a417d7bfa959122153a70f954dff65e1a647efb9883e6c946414e48fe0e9c5a48ad4387ea36e32634fcc4edb297626cd669467247b0784a92472ae47e4acd691e189d7b01eb34962381a8ff76893cf6bdd4c8f87c40b71515dd0654ba71b7dff63f5af7ce04dc57760089bb4ac2a0cbc636685018f770e3229065169f0b4e5a0f1480d4255bb71fc7bc0ee2b538b6247f97f2f73659362a7476883f480e5bff5ea2c503b7b5b870538139a2bf64db90549c97e9baf1ed5eb525ce2c3d6b232a642cfbe47fd4591b6a80b100c01f865924a8f9157a573fda428383beec5fb6e3396e898a5335d76333b2afcfe75df77039ea5f8af0e12f7ce0986c347530de5ecad84aa887fac03f11cc09f1a1b05b5b35a6a359915b98b986b6818f36b3f6054303976dc7dac6212a3ebafc5a67dd47623025426c865674b7b5cdabaccf1c528cfebf18cd0be121ea59b06ce42fcb6ae327f260ac5d7fadd0fac75a37634fc0d6a9217d3671d26e0973ee29d1cad27aa8b9c4a5759a5e1bb05".to_vec(),
        ],
        [
            b"7ec9675098a56979e6c9633b901de996cf325ab4840f87b08a56177eae576d74ea735ed28829fa47147122d123d0c77312dab5681e9dc48cf791125a24c6d70ea8559bde2b16555ed5969a58d85dcf24e1d1763323e9c18d2a830c6fb2e8760476b1cc5c0fb360c207176eced0bf1b208ba1883bfc9db7fc2c212454576076296202fac695e164089c8bf430a760fb526630bd77f912129fe36fbfcfcad75d0fee46a98136d4413c15e8511b4728e9b1ca66e3a0034a4110d13a81856e75ca02eacfc8f49c4695ac7ef2309c88c8cfd960e32a01b6c0fac46db8f8a89f24d005bae3c5c6334e71e54c8af1dc1dd62fd11852370707b939f479bc4b26a342f20828172ec3d82ec90ea58050250e4f5303c0d87e5aed33f84586877d7c202736704ac29eb1a2f0ff98af3baee4c7a1a2ee6d8c495e5b584d3062d20b5e0394a740d6ad56a14d6a041408f9a3cfe711f131055b5f51f6b303b113d2df74d714006abc2b797586d3eded97ea9b78db94fee8cecea9d3d334b3ccd40ac916895bd35f8a3e6825445a3feaa9430a00c611535da2dcabdf3f686bd41e126ff5a18f7e301e060ac8753a6f09d95baec75d19b54444573cddf42787dfd08297d555daa50be4e0756f44dd7f1f3ee1ee280ddba95cec14ebba6043bd356fc29b6a2cde39021717795cc7fb8414894a6d3151e25a92c613756470e56753b365f8a8f40d2303da94a18594f20dabdf8f636e34b668f84d2856c850630c46a2d4462768606a09".to_vec(),
            b"ec36d90690c31ce9cd3b7c755970ebfcfa2714093b15768b6b12c50fb74dee13c6deddfc11d632cdd46caabefb222a19e71b2dfd8f733641118fe36259afba120a019d2698fa20916bd0c48207353e6fa0ad81ed8048284628c793e48ac4e33eecbf2cc250e7e604e50cf02744d5aed9a8823520608e0a80a42cef3da67315622b723f695a73b3ba3371b47a56bc0f1bb65a3c999ef945bc1cd59d39921d2c0ca914607361592d1e42decdcc80ef05abc6680b163936d9f31e0b3a22c842000cdc4978020d2c57704b54efdbaa9bef53ff653b6393554d27c6d5aab50948e70fa2bad4abecf48efea51327ba6d03b24df7d703cfbb286414471fac6ea5e62d67684da7fbaf48b9ad0eeb28cf42a41c741b476136390e9a48e01c1f7220154149502b49ce7b09e3dce1a6027078efc7318288cd07f20aad0ded14b8c5b46b5f79ce60275d247aa33d0f66f501ec1e153b6c0d52380925e27d6ab503056976e444ba07707f7d86918fd8cd4540b51ae60b5d281d590c66f153bd65a13b531e940ef6218b80e2b8555cb8b26c63ae8ac676604699e38ad9db3d7f7441a0b4ecb8382840bceb7fc5185978bb9cc6cec3069963f20bfe37dca1f32c3e20ca95fce60b020e3bc3d8d356a378fd5e3e9f158aab3ad6c1f096ac1c60349bed2cc7ac0a7a4a5ca77e846d28941b794137c722093f5dabc12ea3a95aa1819c21550e497d381a5f2a04b87d83701677a9eba9a07398178e000cbbb3bcf3c8b4d5d38569ff782447595d846da95195de9a765a634c7d57ddc1738972e6c05340abf685228303f4eb9ce78966840e727a746483d31ee2012f9c88e1fc831a1f5746889dea240f".to_vec(),
            b"0ae63b34b0129fc36d5d4214e888b1c69a9d67adda6f58f80f8de74f4e4bc6311af8cf6388fc73515a19d4abfcd91fe93e2323fb6e09a58a51a4d2c4c5eaed18fea816dc01275ef534169183aca5c237187499d8e688be92e1492812d2d0c118e2d36ac8c9f4e1b73ac0425ed84eb447a344f00ded4033a6f10f9bfacbbc5f5131ae699ccc90babcfea39c3142c2fdc7f14315959fd8ed79bdc4a957f9d79f09848272796643e5d142f1a6f6d445f1c936354525af0d3c6731ac1ea66c622d0a0ee123b4f2a5561de8e60269a15e50f3ba11820a71db34c9f3212556d2b64804cad3a0f2cbb3ba67fc802a298c6f78a9cefb9253ff8486072afdada6c97a0a2102985979cfb6f70e5acbfebcfc260d4258a4ea9c4ad4fdbca9d12bc1ffefd13774921fbacf2bb4826aabd52a16b4b6a795fb31d25a1bf9267108e10c2568c754dac540c71bbaf8039be555ec6a9c58efdce197bcc34930d93398da0814c4217ab43d3d7083e80983e675f4b5b474b58ecffff7bc23fa568a7f7ce53e60c9704ff6964476e3495b1c84e3791df5772c2be858707bef0839c3ebf7b6ff29715e25083ba6d6715b0d354b9022333eed15608e730f089c196aebaab5c039cb9de94e4ecf630621d44d586d9108a94d24ddd33bb92c882ddb1cd0c419c9408e73d06ccea91e12c4418f3d8f8c89b5631861f41dc328fd110ab23455b2265ffb122e36da1f7205cd0f7c01a0a946d50d09aff1caa31d7a06d433e0c79498aae4682c2996a96a3762ac2c3a514e290dee08969c27d8c003ab482f50da2d1e4ab6517c5fd6ea8caf2ebf2577e0ab49418ea22bd99b5293676c3daa9c1c98b38e705ccf612f6a372284fffc000fbff66601e14e57842efb755950ab661fffa2d11abe990429a6aed8d84e4cba63109a8f1f6ad137247d2b38934bb234b6217d2601f5110b".to_vec(),
            b"46778375200c54d5310e77154cb25d655bd92e19e247bd709490fd057d3c3047faf72b6df04969a80cbf6abd9c0d7cb4eaee6d1bd9b58497eefdfedd6d9ba6670841cdea0f95f5720513f9d22e1a48c2a860b9673e7c4c6578d6a64418bcfe58bc731c96bc54eba127b8df4c86523bafcf2e78ea0644f5eca2bb6948752327532e27ee56f4ba480cbc9589456a1bb67a0b281785d7343cd1107eb3694dac81088667ff1dc2caa0a7f63a071571ffece9cdc36bb19672fbcbd95a818d872afd05abe6f936c6b9fac685d2966d5da3409aba0bb788f5a02dcb17235986c99dff049ce12d3958ee844555d207e78d0959b6c73bb61bff3116c6202095a711c5cc7b14d57b04ec80a95d39a0e58b3d010514e8bca3611c8cc4ac1be68a96ead0491ca8c086f8ba6a00ebb81f3e0a00f8594d1b1d6844dbc67d172b8c228a307b0256026514fe748b290c8119fda02b214baaa789014829c3077e9289e824f49d867f543f7476cd52cab01947507716ca08b56951bc78832eeb15fb8dcad9074e2645a6e41ed562d10c375a40a8635c64b769ace2c0219d1750614e462cdeb2e3d63d444aef6d0a97a43fed86a4626faf6ff2a7866064e764f2d843593a64c3b0141e021855f4e9d2f5e324074ff8b23be461a99c5e5c0934eb0846d2f99ed07a985f6c6b061191dd4046da6640816a25948a22342e7bf601160d6b91e3a78233f210f06654f3b1a80428aa52157060348b34ccde0039aa528ef362292d9ed295485cc8a4629d6235e27988566345f905014fb216264fe7c57d85a0e26fe5d876a9645ecdfc5b214bb0250f606ce7be72edbc551b89049e5a3891e014b7c9c959691cc2503ec53d175471ce0d98473d1b694561eedc8816dcc10bf75264ae08b6026b36a8d4b78cbd3934601fbee870ee46295be958eefb8fd5f3fc6a1b51aea42a1dc39c09560e2fee1c06a4e86fa11aaa4fa3902a567879ac4236ebf49d7b9a5800e274cd17ba1e98c61df9c87b4e60da414b4d259b848cec54f7ae47572fe48708".to_vec(),
        ],
        [
            b"9cdc1ed1e83ada128d182d30575bc437ecbeba58cf9faeed18fc2bc359c6d227a265cb521b3dbe2e289e3d68efbae1531708fedb6017d4fbbc9b747ee5a040565af225182509cbacee851cb3d2fdf8bf6601419b0ab262238ccd549f394b0a081e4bddc747ed1765a75db0ca54a05d8eb54fb87c7071849d248ca2df4c75ee50dd7558fd070b1ae72860b2d2c3e6966b4f0863d57ee93ebfc25116bf949c2b047e85e4b028ee18f0a43b8ad4f7c38f59ebe11a6289203387c01291bc32c99f01ab6fab5b6f970f0e9d01ca4f174e7d4fd5056e964811838831fefb6671cf5407624e47fc555120012295024d1db3972db8377587f8690cac55ef27967ae27310564cc99eb0bd2e935826d47275c7d4878f836c970dcba0c9468c82c7e84b9912580d16b60b0f5c67650ea9de49f18fe81bf4bd85dcc1a8f092c672baf8336335cc927722200f0e108c8179d01063543907d699b102370a93a14995d3ead9f46dbc133563e7b1a8ebe2b96781ec00ff4a22791ef0f4fe2508268159edd8e5b210304207771e1ac4cd69efc58bd638b039509f9e39e18ca5625fd6a91d496b467f702b2a82f46c9b42f9df3680eae5095a11dfa4724fc7b1cf8dd52ffdba97316ea4acf5617646abd8a6179af60b7e30301f8363b6641bf2a0346378e3655bf41d4c4b16dedfe7a6600062afacd2983875b5fb2341e85a0defa33161e5f93c9030461ccc252c804c531a070d7dd748f0086a67f5c85f9049d38409130282bab06c4442bbcc54e9f3b0c5e7f20e1421060a829aec404367e2836fdbbd07febbef0ae26153deaa93067d669dd0ed2af14d7c8a2c2f66e973660096420b7d5e570102".to_vec(),
            b"d41df673b989900a92520255a001bb4a4d1457ecab5f9743a956c2fed93b6c517a761e2b266affa6fd86cd9d1823c9face4fe8a4cc70c3833f4fd87f415f0d11e86ce86e66656c6cf65e79e42b6c5bde347a88b77f24f4e59bd8ad4c34179a75f0c6f6c27eab1463f7d6a17c90c300bdd5433ea8afa0ed45e1e4ddfb2f1661746b24b395fd3250d3d2eecf04995c7d19fd1bec259502e5eb5da4ae1f387f8708f5780deba40aba64b6f99b8a9a1066e803671fd4fdc36f67243e28bc50c75d0993394321a82514791c929da1f30177a5a203f03028e5c2d0d1ca32a2d8de0f0e8405b523c90296a7a089fefebfabde4a0a499b543e399c239e47977e4317081054ed703b4102c1e6d20ce84c614ceabe862842a5b0c22209815deb8da6e6100c626ce0d51581443bf83d9bf67195f8f6420ac77884cd8ce018a6594b3777e92608bbd26a10fa4182bbdef249f619e7140f80cecd2478827479912478eeed1d48d8ffbcbc10fdbf46e50c282bac6e53a909c2947b8cfaea2bc1d532538f322473e6be6616dde8f410c4dc05a2c5e57b75a43b95265e78ee710ece5f9371d9eb3b1ccf0d135dee3a5f0d91064127164a7965a5873b64de599d296f2b469160ee51a2be9b94772bcf30d6297d75d279f30715629479785eae3a0ba2ece3c3abdc49a0f043f2577d711a3ea2bcae8c16bde426df6c5d5cee246951a955d708ba716f0e6d72c0efed05b7c2cf4f4fb7743b66913008d22b6fd2b9f671a9b2318f162928a6ae69e1367669dd3acc71ae54fafe39b36049cab12095e5d3d33300ea931568afb3977d0dfd8f807a7f1fa9145587dc6dc5b692b1558dfbe6454a17edce5a795493c3af7b34f2930b09aa16c3801ed671c4b8493b7576493cef0d09cfec0dad690cf5ac490c7b3b97db915253195a47e9e0a02eb168c9e10ea5cf30b6210b".to_vec(),
            b"06ab929eacb52f35b0edd375c1e28c9f3366b363b1027123b92d8eb0e92d4b1eee687415b1a09eb55874c5a8149648361acb6f33f019684690a9fd40d69d3c5a562bc4f17c68a04cb46c2abbc5bfb9474e8535845a32dd1fde50cdf387c94b454ab96a379cbed05468ff7f98f85cec7947efceb75603d009f5a31937a8ed153ac4cb9a12ebc431a65301795b4e7060825bb33a451502a0f7edb232de205e590d817c780c841295860b6028e4bcf35e14db6bb674802c5b39f9bc99ebfad19205f4086b32b535caccd6bdfcc65cdb6a9240abba199e2ae2b8ebc7d9e9bfabea0d6ee4c49e3fde4da917b2b41f3ebf7727d0f29e31cc5d33ca9714f687da4c315a3617dc9b823b2210b8f9c703c0a81b5ebf6fd8e72ddd05cf66c728864c1ee946223f16528897de08346270a163a40e424b9693f55c3a36159614cc0e51ddb61e08c6075ef3c736ae7f070fee3c043bd296fda6a50b8de597b0e503bc1deaa95d0e8a4038116ef803aa40b955db4859744cc43923cdc9a90c233785932ecb0403c26c0a49527fd4ed8c9c4ca30cf230ed908926d7c35acfd9375e7824a8603b068e517e172a8300b77b27f1013ed286a23ce6d7afce5f6215a8f2b8c7c13d7b2fea527c688c46bcc6df5ade1f95e4f4f3772d6d5f77785eaa703a621ab05d194450a33f223fc8937d21c936f2d9e6f4ef58884e1ad3107c5c2e459fb182b6e7350466ceb8eef5286473629fb96df8390f75f18d58d657c5ddc7a8c8dea250c549b273bf416e287973db5dd1c1b1bcf2fe1dcf57f32ca5d040b3206e4141343737328e8a0887fa748d597973be204f96c9b395ac40e6d085eec62af494981a215a36b38a58c0d63d28e075c95d35a080fefe00287533e1c2cd31c750ebfdf6112c8c6fdba47cdfa5c3d65ef288f43bbca9b8e7c7ca581caad65c551342f8689766f377860a82b0cf62f3866d716fd74d366589ed02eb4c1ac2cda48f41f1c3240bb05c7d9d182a3a2d2686049758ebe45a153e137bad00a3ddd0351c0a862fac03".to_vec(),
            b"3a1454aa90b807e350a7eed14159aef5b1658323a2fc19d4819397c0f0cb9b06ec4cfc8b4ba21f9c4ed7d54378bc6ea68638344179f50bc3f0d654183f6b3743d61a05ee13614428d048c34caed8e15c780443cf3c0bb69e4e720d8330ad47376ea003938954d0863f3984d1b277d5dde6f73b4fa9a002cf81f8ae0b6de4f66970d05dddf2cb972771f221a895b3228105bbd9279fbb82aa215253c144a5ac02ec36298cdf9278cefa0b69332adcde7d8a0b12f3b07c9628550f88a2e69fb200e34389f3712ecb2d32c0f0977a85a4f33d202edfdec7ba1c2d5016c88605380e3471b8b86982fb4719aa498641e3870a5689faf0b8cddeaf7976c2cda9e9095034c2cf3d1b5734beba84c7003309981e49ab7b544f8ec382b55ad188146d231fa6e125542e5fbb4c9b1219d9b90d8d3b16e30b74024ebbdb5c1b8ca8084c4f53e2076328268fd767684534f97b745734c901753ef3708d8e40bbfc7f669bff761c77f38dff0cb69654decb29a2e8e3d99d920ee03d0639ccba354682741ead7706d7cdef25cd3dd20ebc283fd09a6a0f1f1f41e1bd2a49458913b1ceb26cd768349581d6b0606aaf1acf1fda7836fb66ccebe8787ecd212b2a27e9044f78d70e108c89ad1d2d4953083d617c97bb47eb845775d494bbbbe998e12f56d4d03d1e668e466e962f6d9c6645dc6a8ea93639d8c5e963bb967b17dd0d34747c5e1d4dd463e201b994088fd7ddf6129a436459bcf41b14607242ed99fd537c69592d22d0d07555b8ccb480853ceecc02dd153c9c2ef0f2187811fc614cecd9c0586c5e187101f10aeb6288bd709e5728584c3ba1218592e5d0a88d942df801146b9d48ee4ca2b01381f5046b05eda95b2fcc1ee26cf97ee55bc1b49edef8d71e070e199cc3db42f295b905b08312ea944fd4702463cafbb4e133a193367e8b886aff4e0ead3ac0d722aed0624a45e9314efbc2619a476edee0cb4fc9470b20eaa9834f36681ca232fdb4dd046804a54e82e95c438e01f5f277941bd92bd5921f07934d23cf5ac900121fc8576723a2e753b90c0253f18621cab4cc9815a834340fcd0b1829918391cc28ff6d420739e1c3d7e943f20b87a6609f30354463d9ff46dc0c".to_vec(),
        ],
        [
            b"9acbe03bdd0b3b663d4fe3f7a6ff57060042ea418aee184d23074ab616548a255c67c87d49d71829c29df85b0b13175469017f162ddd626abcdf49a79aa8ab5e00bb83bd1813739eb0813ea975f601adb82064cc7ac7ebf36874eee0c7a90e2e48529e7cb384ab4d36df9b3f8a66e953f0f2ed224fbbc6402cd4e56910e8a13c6ff62d272dce553acc040f40c94bbbb4ac827b91e35cb2c7db0a6cb2b8073302cce49751d2c97e89eb360d91d6ec6b6272344d7a1560a12c5f6bd9b129a9a00ceb8e1dadcd01d3cc4fe294036ac997a9fcee4617dd39480eefeb7be5c1e3bd057272f721bd11352798a8792ef7b41204ef2450bf7f353eb51525c4d3c7d7ec52dadd8b87614ca5e9b98a34e6fd79fcb35c40d0420bc378fe0bdf15c9630cc10264395b946ae0466b8d4dabf4a11c2bf630c72e469c4f347ff72c6ac7e5767c6e8679ac78a51e79de35c2d02da8dab7b7976e62e900355366a2574f008302df3692bde4df4bd157410f515864a3147eb4b28a9151f3600b710c47ff6d08b3561678c9d16cb27eb8e1a2909d80976a295286bc383de6816c283303d41ac3b68e54060f9568114ea89040a50c6e7b505d49359988a859e0c867e190d8e58aab9a052a9a3ca7d72d1ce088a8448812047adb70928c1bc7341ecbb514c074e23a440b62832883fa6db8d151cd265fd228bb2337f0eacd7d001482debd626f4fee815f3a79aa848e4468426598babc335d0875b435981b2348a3a28b70b536e3f04738fe0553207e1d5f1c0f97e3164d0fa3eb37ee198905f2e552ef3e6e3083b19002f866728d19ca3c918d18c5540a9c1f930767e63bf79d038087a9b92a01f94c245aafd531cd2d7bc20e2ac0ef9a30c5cfd82747afada0aea682ee103a88fd1101d9c97c5a2859e728c18c9bba4003d99fed92cf6b9242651d75348db6fa0dae03".to_vec(),
            b"7ed1b8dde36bf733666dabb42f65de0a3c12fa02b2352e07358b2b69b89a3445482f33b6d3b964a9fa06e04263e25d9f9e74e3789abdd366b55544882996da1a0aeba5165d4b52e3da7d6526a390518e9f655fd0d6e20013f81522b3f02ea31c1a1d400d63b38af7feb1c017fc068a3e58f0e15f5f570ece29f004f420d7c556abbcc01498043c4ff66fc528c8d6edf5536739f08497b085c711614f90c6cb0a99f5107f862fdf7b1801a91da1b2aa9761d3de42d88d1187d2b580a4c1f7540423e35f5cd1255bbc7bfc2e8e0c9c8a778aebeea1c0b2d4b658653f1aefe3d207a84e421db6d1a6673df8432173874db876e11168a6ca90ab32350f667a26ae577640d0b43fa0fe3e2a9f20762d892fb3ec6a2ebe1a9768e61a902446f90bf17de893b31d9632afc319c206d7df551acdc3aa2cbd079b24c2f78f818e2b66b93fdc683723a55c6a40f7d65002333e66696d122f9b15a925cc17f9583440d7bd3ec8194da65cf2e91e1cd1fdb1f9c41f9305230dc423c96abe12d967ec582beb3322d94bd1b470c3c4a2e872e9be876b19c15fe66f9791e533913a684ab9b0084112b6b44453d3404c3f6ce7d037984353a1229dc731c085541b357500bc9cb85b5ebd423d136f99d8d9225d4b56a1d4f5f7dfd28a003c640544f5040182b54b0f0a43c78c910b8e2a80393add806d2cf13cf05a8995a2102f5dd6bee34ae01b28143097687ac3cd4a0cc4fb4068353ea6964440f04b7379e6da266d3305d4d74d6683fa2180d29cdbd09abe5c9f67fa29a4d9d6ab3d90683cb0269df3066f4f67d67fe5384f736a4bc70c6aa22f7dad32cff805888288eab9d733c97878766d62c0e34d85c25fd7b8e30f506e038193c5ce92794c2932f47a8994bdecd669327452a0f67024fb9f719a42c6c4864cfbed4780a740a2b2a96f8e48bfd34d8005675b0828f06c1b1e5411ad87583810a0593377114b10711214e63de2cf23d41b01337021cead358b7ef8472cd2c500756bd66cce0ab6256146d79111c4ecee5004".to_vec(),
            b"f2947845b1d358095e22446e08c08dd4254dc6df6b107e65da8a22c744f2b23b2694658c5b3bac812e397fb9e51445064298e8498eef2f3a67940b0239fedf627a2b7a0f4834ac6c59979582b122b8ad11a42d9484228e13c8702af9833ad8261e02b62d6c63aa8638020d57823e70419dc9bcda09e698ab8a444c815d8038373691e67cb9e58d1951d6e40a711a83376ee6e03af83c3ab1dfb9bd69874ff00b384b4e8896aeb3f4443b95b6704281a19760f596b1063c328fe9168ae9169b048744f3905af3c67b6679e14d5294b6aa8c671edda152e4da70fa9571fa80660d6aecc9767fcb8ad80fd4957e01e831bda17ec76ceefbd9de562288c09c72486a3476c1eb86a63701811996a309015020ce408d9ed4ba7be0440297067813480aac45a9d214f3aeac5ca96463a96a3a15ac89b4e5f75dd0c1f12733809df11f73108d97bfbad6837a4bae88ba7487d13f728bab851331b64fac8bc6e742db1429f6e4819971979c139a208a0edd0c17f96afe664b4bcf39df5883cdaf7c464423aaec059d0b1599fd369a16ba72da2ca164c28c8bb313952a366d17ec241d5a6772f010c2e1fab05417c234f5ed4fa822c49c6c0f3da50092de007d5f5bd4825e7670bed3804d165abac1194ed8cfa00cfa251add7bc2a41cabf06b48de2875567251c31dd105829f0f3218b0a39f30bf8ff33060907ccd00cf68a1af1c920a696cbbf49fdbe59b52b1fb359338b007e7bcc6a4099da9a71d3d9331d5e5b8fc192a28083662c03f4234b2d3ece016c19fa40d8c6993ceb48a7225488e6f6d1904ec460b09d3b7bafc4aef80f206c98033d4127702ed1d3c14fd6175d0d484cb558852d38e4817e9ce4d45e776291c2c9d26217b1fb6cc5ae2acf2c8b6b3902a0b023b1ade6a6140f829c8fa6ec9772d2034f28f8b51a91daad5b00772d432ba27363a51fc73344457e8320388cfff8fb142cca55ae18f6ad157de19c5de75cb7230ace03eb3c0d23885a44820eda9cfae377ff318c3edd25c5fe10364882fc91d236c80ebe903f0dd986542762febb2f055a7c23b863d6064a191b33a3aee1c023920c044c0ddbc9e7d4c677b68d584d88d16cec062641e101385a2a95d196905".to_vec(),
            b"9824fa7f8dc4e219fc9dbd1d3d8c0a30db28b98f6d57c9601a07541d5a6a423eb008f517eff83f041a65cc9c7795d77c3528d1424f77aee2c3bfe78e4b5a08705e6412c8c6f64527f33e6bb81b9d2c227798321d80ea4bec9cf91e50b7b1e301ce79cc816cce3fbe6e9bacec0127af5ff35c84ce9fb8997df913ab74b27a26278ce2bd133e687f691e9c97c065ddf051f13d54b6c4781eeba59bc0651802ac0b676a125d6c0d7c36bea5e430333c533bc9447f4823afb555154fbfae66ce460295585fe54971380d3d0ae80ef179f690600dfe383a267539153c270f54278004c6945290ee6b2b3b2e1ea1cef8c7f0e5334a1801d6f4be804689dae7f080cf5918d56abdff71fdb1918badd77f5c2953b1399d750e34b46cb16ae6eb0afffb1d5ad902e520294bc50691401637fea4563b3d800d2445142061d40586c6259e6de608b25f45ac84553c8ac6a35f9605ca84ddd8514217ba76a86800bc22090200e460a1e1f6f708135b2c673c913b732e895ce592627492e76831842fdbfdd73f38e949c6cb2201da4800a20588e7b6ebec720b899d63761540d461e02dd5bd484225da7c545578b9e091017e58d6a98d5d6b0331ac05378ed4719e9d2b8e31295af78863e169c83f50eb014bf1d041359ad3d2309504691e3a19ffed4c02251c30cd70a6e5e37dbf16414122958b1aa532a76fe9f03885a2caf1624aeaad916b627df349744ac4f7012c449b8498fd951c4fc5ebdaa6197be557e7d7278d216b922558c4f0fbaec68613c08b3e11f6dd9e1fc7ad88ff6f2c1ac8f6f733d0683e7e15959a5ce7ed95864df9451b3087be383e1d86d76c80193ce5d4b2af1dca740ed1531bdfbd09320d864981c7147b4e601c58fd8253698e303cb4ea694b7e060c589c8222d653f6c88fab9d7d1911b9942e607cfacabaf54bb3b3b9c499d5638a0d165cd3b49c7af74975ee8b570e46f4933e2f8ae5228e6324d08971be027a78ae9664e210f28b1a90165b92f0343a0d0837c0ba3b0cf508c26b4662b65970a2c170f0cbdf31c9c392ab7323620f36cc526c07d47ec489560ba2582d6b3038840ebf2b74ca356c81173302c38f40484f6b6b4bb0ca763b989ea0081d4e5f7ada8b4a5d989662f7d28009c13348e1a718cbe1ac054fe51295a91d779c5b880aaad97c2abcc5e05cfb9959570f350ba951a5ed6a37c079683c43582a5dc1fa0b".to_vec(),
        ],
    ];

    let vc = [
        CompressedRistretto::from_slice(
            &hex::decode("90b0c2fe57934dff9f5396e135e7d72b82b3c5393e1843178918eb2cf28a5f3c")
                .unwrap(),
        ),
        CompressedRistretto::from_slice(
            &hex::decode("74256a3e2a7fe948210c4095195ae4db3e3498c6c5fddc2afb226c0f1e97e468")
                .unwrap(),
        ),
        CompressedRistretto::from_slice(
            &hex::decode("7e348def6d03dc7bcbe7e03736ca2898e2efa9f6ff8ae4ed1cb5252ec1744075")
                .unwrap(),
        ),
        CompressedRistretto::from_slice(
            &hex::decode("861859f5d4c14f5d6d7ad88dcf43c9a98064a7d8702ffc9bad9eba2ed766702a")
                .unwrap(),
        ),
        CompressedRistretto::from_slice(
            &hex::decode("4c09b1260c833fefe25b1c3d3becc80979beca5e864d57fcb410bb15c7ba5c14")
                .unwrap(),
        ),
        CompressedRistretto::from_slice(
            &hex::decode("08cf26bfdf2e6b731536f5e48b4c0ac7b5fc846d36aaa3fe0d28f07c207f0814")
                .unwrap(),
        ),
        CompressedRistretto::from_slice(
            &hex::decode("a6e2d1c2770333c9a8a5ac10d9eb28e8609d5954428261335b2fd6ff0e0e8d69")
                .unwrap(),
        ),
        CompressedRistretto::from_slice(
            &hex::decode("30beef3b58fd2c18dde771d5c77e32f8dc01361e284aef517bce54a5c74c4665")
                .unwrap(),
        ),
    ];

    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 8);

    for i in 0..4 {
        for j in 0..4 {
            let (n, m) = (8 << i, 1 << j);
            let proof = RangeProof::from_bytes(&hex::decode(&proofs[i][j]).unwrap())
                .expect("Rangeproof deserialization failed");
            let mut transcript = Transcript::new(b"Deserialize-And-Verify Test");
            assert_eq!(
                proof.verify_multiple(&bp_gens, &pc_gens, &mut transcript, &vc[0..m], n,),
                Ok(())
            );
        }
    }
}

// This function generates test vectors and dumps them to stdout.
// It can be run by uncommenting the #[test] annotation.
// We allow(dead_code) to ensure that it continues to compile.
//#[test]
#[allow(dead_code)]
fn generate_test_vectors() {
    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 8);

    // Use a deterministic RNG for proving, so the test vectors can be
    // generated reproducibly.
    let mut test_rng = ChaChaRng::from_seed([24u8; 32]);

    let values = vec![0u64, 1, 2, 3, 4, 5, 6, 7];
    let blindings = (0..8)
        .map(|_| Scalar::random(&mut test_rng))
        .collect::<Vec<_>>();

    for n in &[8, 16, 32, 64] {
        for m in &[1, 2, 4, 8] {
            let mut transcript = Transcript::new(b"Deserialize-And-Verify Test");
            let (proof, value_commitments) = RangeProof::prove_multiple(
                &bp_gens,
                &pc_gens,
                &mut transcript,
                &values[0..*m],
                &blindings[0..*m],
                *n,
            )
            .unwrap();

            println!("n,m = {}, {}", n, m);
            println!("proof = \"{}\"", hex::encode(proof.to_bytes()));
            println!("vc = [");
            for com in &value_commitments {
                println!("    \"{}\"", hex::encode(com.as_bytes()));
            }
            println!("]\n");
        }
    }

    panic!();
}

#[test]
fn range_proof_rewind() {
    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 1);

    // Rewind and blinding keys
    let pvt_rewind_key = Scalar::random(&mut thread_rng());
    let pvt_blinding_key = Scalar::random(&mut thread_rng());

    // Commitment value and blinding factor
    let confidential_value = 123456789u64;
    let blinding_factor = Scalar::random(&mut thread_rng());
    // Extra data (up 23 bytes) can be embedded in the range proof for later use
    let proof_message: [u8; 23] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
    ];

    // Create a rewindable ZK proof using the two private keys, also embedding some extra data
    let mut prover_transcript = Transcript::new(b"Bulletproof-Rewind Test");
    let (proof, committed_value) = RangeProof::prove_single_with_rewind_key(
        &bp_gens,
        &pc_gens,
        &mut prover_transcript,
        confidential_value,
        &blinding_factor,
        64,
        &pvt_rewind_key,
        &pvt_blinding_key,
        &proof_message,
    )
    .expect("A real program could handle errors");

    // Verify the ZK proof as per normal proof
    let mut verifier_transcript = Transcript::new(b"Bulletproof-Rewind Test");
    assert!(proof
        .verify_single(
            &bp_gens,
            &pc_gens,
            &mut verifier_transcript,
            &committed_value,
            64
        )
        .is_ok());

    // The two rewind keys can be shared with a trusted 3rd party, whom will be able to extract
    // the value of the commitment as well as the extra data
    let pub_rewind_key_1 =
        RistrettoPoint::from(&pvt_rewind_key * &RISTRETTO_BASEPOINT_TABLE).compress();
    let pub_rewind_key_2 =
        RistrettoPoint::from(&pvt_blinding_key * &RISTRETTO_BASEPOINT_TABLE).compress();
    // The rewind nonces are derived from the public rewind keys and the Pedersen commitment
    let rewind_nonce_1 = get_rewind_nonce_from_pub_key(&pub_rewind_key_1, &committed_value);
    let rewind_nonce_2 = get_rewind_nonce_from_pub_key(&pub_rewind_key_2, &committed_value);
    // Get secret nonces from the private keys - this emulates the owner of a wallet
    let blinding_nonce_1 = get_secret_nonce_from_pvt_key(&pvt_rewind_key, &committed_value);
    let blinding_nonce_2 = get_secret_nonce_from_pvt_key(&pvt_blinding_key, &committed_value);

    // Get Value Test 1 - provide correct rewind nonces
    let mut rewind_transcript = Transcript::new(b"Bulletproof-Rewind Test");
    assert_eq!(
        proof.rewind_single_get_value_only(
            &bp_gens,
            &mut rewind_transcript,
            &committed_value,
            64,
            &rewind_nonce_1,
            &rewind_nonce_2,
        ),
        Ok((confidential_value, proof_message))
    );

    // Get Value Test 2 - provide wrong rewind nonce (this does not produce an error, just gives back garbage)
    let wrong_nonce = Scalar::random(&mut thread_rng());
    let mut rewind_transcript = Transcript::new(b"Bulletproof-Rewind Test");
    assert_ne!(
        proof.rewind_single_get_value_only(
            &bp_gens,
            &mut rewind_transcript,
            &committed_value,
            64,
            &wrong_nonce,
            &rewind_nonce_2,
        ),
        Ok((confidential_value, proof_message))
    );
    let mut rewind_transcript = Transcript::new(b"Bulletproof-Rewind Test");
    assert_ne!(
        proof.rewind_single_get_value_only(
            &bp_gens,
            &mut rewind_transcript,
            &committed_value,
            64,
            &rewind_nonce_1,
            &wrong_nonce,
        ),
        Ok((confidential_value, proof_message))
    );

    // Rewind Test 1 - provide correct rewind nonces and blinding nonces
    let mut rewind_transcript = Transcript::new(b"Bulletproof-Rewind Test");
    assert_eq!(
        proof.rewind_single_get_commitment_data(
            &bp_gens,
            &pc_gens,
            &mut rewind_transcript,
            &committed_value,
            64,
            &rewind_nonce_1,
            &rewind_nonce_2,
            &blinding_nonce_1,
            &blinding_nonce_2,
        ),
        Ok((confidential_value, blinding_factor, proof_message))
    );

    // Rewind Test 2 - provide one wrong nonce
    let mut rewind_transcript = Transcript::new(b"Bulletproof-Rewind Test");
    assert_eq!(
        proof.rewind_single_get_commitment_data(
            &bp_gens,
            &pc_gens,
            &mut rewind_transcript,
            &committed_value,
            64,
            &wrong_nonce,
            &rewind_nonce_2,
            &blinding_nonce_1,
            &blinding_nonce_2,
        ),
        Err(ProofError::InvalidCommitmentExtracted)
    );
    let mut rewind_transcript = Transcript::new(b"Bulletproof-Rewind Test");
    assert_eq!(
        proof.rewind_single_get_commitment_data(
            &bp_gens,
            &pc_gens,
            &mut rewind_transcript,
            &committed_value,
            64,
            &rewind_nonce_1,
            &wrong_nonce,
            &blinding_nonce_1,
            &blinding_nonce_2,
        ),
        Err(ProofError::InvalidCommitmentExtracted)
    );
    let mut rewind_transcript = Transcript::new(b"Bulletproof-Rewind Test");
    assert_eq!(
        proof.rewind_single_get_commitment_data(
            &bp_gens,
            &pc_gens,
            &mut rewind_transcript,
            &committed_value,
            64,
            &rewind_nonce_1,
            &rewind_nonce_2,
            &wrong_nonce,
            &blinding_nonce_2,
        ),
        Err(ProofError::InvalidCommitmentExtracted)
    );
    let mut rewind_transcript = Transcript::new(b"Bulletproof-Rewind Test");
    assert_eq!(
        proof.rewind_single_get_commitment_data(
            &bp_gens,
            &pc_gens,
            &mut rewind_transcript,
            &committed_value,
            64,
            &rewind_nonce_1,
            &rewind_nonce_2,
            &blinding_nonce_1,
            &wrong_nonce,
        ),
        Err(ProofError::InvalidCommitmentExtracted)
    );
}
