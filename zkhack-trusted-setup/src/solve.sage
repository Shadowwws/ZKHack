field_modulus = 4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787
desired_curve_order = 52435875175126190479447740508185965837690552500527637822603658699938581184513

Fp = GF(field_modulus)

PARAM_A4 = 0
PARAM_A6 = 4

E = EllipticCurve(Fp, [PARAM_A4, PARAM_A6])
E_order = E.order()
assert E_order % desired_curve_order == 0
assert desired_curve_order.is_prime() is True
E_cofactor = E_order // desired_curve_order
Fr = GF(desired_curve_order)

R.<T> = PolynomialRing(Fp)

# Starting at -1 is an arbitrary choice, could start at 1, where 2 will be the first non-residue
if not Fp(-1).is_square():
    non_residue = -1
    F2.<u> = Fp.extension(T^2-non_residue, 'u')
    for j in range(1,4):
        if not (u+j).is_square():
            quadratic_non_residue = u+j
            F12_equation = (T^6 - j)^2 - non_residue
            u_to_w = T^6 - j
            w_to_u = T + j
            break
else:
    print("Unknown")


F12.<w> = Fp.extension(F12_equation)
E12 = EllipticCurve(F12, [0, PARAM_A6])

E2 = EllipticCurve(F2, [0, PARAM_A6*quadratic_non_residue])
is_D_type = False
A_twist = 0
if not (E2.order()/desired_curve_order).is_integer():
    B_twist = PARAM_A6/quadratic_non_residue
    E2 = EllipticCurve(F2, [0, B_twist])
    if not (E2.order()/desired_curve_order).is_integer():
        raise Exception('no twist had appropriate order')
    is_D_type = True
    F2_PARAM_A4 = PARAM_A4 / quadratic_non_residue
    F2_PARAM_A6 = PARAM_A6 / quadratic_non_residue
else:
    # E2 order is divisible by curve order
    # TODO: get cofactor
    B_twist = PARAM_A6*quadratic_non_residue
    F2_PARAM_A6 = PARAM_A6 * quadratic_non_residue
    F2_PARAM_A4 = PARAM_A4 * quadratic_non_residue


E2_order = E2.order()
assert E2_order % desired_curve_order == 0
E2_cofactor = E2_order // desired_curve_order


G1 = E(0x0F99F411A5F6C484EC5CAD7B9F9C0F01A3D2BB73759BB95567F1FE4910331D32B95ED87E36681230273C9A6677BE3A69, 0x12978C5E13A226B039CE22A0F4961D329747F0B78350988DAB4C1263455C826418A667CA97AC55576228FC7AA77D33E5)
sG1 = E(0x16C2385B2093CC3EDBC0F2257E8F23E98E775F8F6628767E5F4FC0E495285B95B1505F487102FE083E65DC8E9E3A9181,0x0F4B73F63C6FD1F924EAE2982426FC94FBD03FCEE12D9FB01BAF52BE1246A14C53C152D64ED312494A2BC32C4A3E7F9A)

G2 = E2([0x1173F10AD9F2DBEE8B6C0BB2624B05D72EEC87925F5C3633E2C000E699A580B842D3F35AF1BE77517C86AEBCA1130AE4,0x0434043A97DA28EF7100AE559167FC613F057B85451476ABABB27CFF0238A32831A0B4D14BA83C4F97247C8AC339841F],[0x0BEBEC70446CB91BB3D4DC5C8412915E99D612D8807C950AB06BC41583F528FDA9F42EC0FE7CD2991638187EF44258D3,0x19528E3B5C90C73A7092BB9AFDC73F86C838F551CCD9DBBA5CC6244CF76AB3372193DBE5B62383FAAE728728D4C1E649])
sG2 = E2([0x165830F15309C878BFE6DD55697860B8823C1AFBDADCC2EF3CD52B56D4956C05A099D52FE4545816830C525F5484A5FA,0x179E34EB67D9D2DD32B224CDBA57D4BB7CF562B4A3E33382E88F33882D91663B14738B6772BF53A24653CE1DD2BFE2FA],[0x150598FC4225B44437EC604204BE06A2040FD295A28230B789214B1B12BF9C9DAE6F3759447FD195E92E2B42E03B5006,0x12E23B19E117418C568D4FF05B7824E5B54673C3C08D8BCD6D8D107955287A2B075100A51C81EBA44BF5A1ABAD4764A8])

mod1 = (desired_curve_order*G1).order()
s1 = discrete_log((desired_curve_order*sG1),desired_curve_order*G1,ord=mod1,operation="+")

ord_g2 = 13 * 23 * 2713 * 11953 * 262069 * 402096035359507321594726366720466575392706800671181159425656785868777272553337714697862511267018014931937703598282857976535744623203249 * desired_curve_order

mod2 = 13 * 23 * 2713 * 11953 * 262069
s2 = discrete_log((ord_g2//mod2)*sG2,(ord_g2//mod2)*G2,ord=mod2,operation="+")

print(s1,s2,gcd(mod1,mod2))

s = crt([Integer(s1),Integer(s2)],[Integer(mod1),Integer(mod2)])

print(s,s.nbits())
print(mod1*mod2)