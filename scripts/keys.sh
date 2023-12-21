openssl genpkey -algorithm RSA -out private.key -pkeyopt rsa_keygen_bits:2048
openssl rsa -pubout -in private.key -out public.key

cp private.key ../client/src/lib/server/private.key
cp public.key ../service-users/public.key
cp public.key ../service-notes/public.key
cp public.key ../service-utils/public.key
cp public.key ../service-utils/public.key
