#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const SODIUM_VERSION_STRING: &'static [u8; 7usize] = b"1.0.16\0";
pub const SODIUM_LIBRARY_VERSION_MAJOR: u32 = 10;
pub const SODIUM_LIBRARY_VERSION_MINOR: u32 = 1;
pub const crypto_aead_aes256gcm_KEYBYTES: u32 = 32;
pub const crypto_aead_aes256gcm_NSECBYTES: u32 = 0;
pub const crypto_aead_aes256gcm_NPUBBYTES: u32 = 12;
pub const crypto_aead_aes256gcm_ABYTES: u32 = 16;
pub const crypto_aead_chacha20poly1305_ietf_KEYBYTES: u32 = 32;
pub const crypto_aead_chacha20poly1305_ietf_NSECBYTES: u32 = 0;
pub const crypto_aead_chacha20poly1305_ietf_NPUBBYTES: u32 = 12;
pub const crypto_aead_chacha20poly1305_ietf_ABYTES: u32 = 16;
pub const crypto_aead_chacha20poly1305_KEYBYTES: u32 = 32;
pub const crypto_aead_chacha20poly1305_NSECBYTES: u32 = 0;
pub const crypto_aead_chacha20poly1305_NPUBBYTES: u32 = 8;
pub const crypto_aead_chacha20poly1305_ABYTES: u32 = 16;
pub const crypto_aead_chacha20poly1305_IETF_KEYBYTES: u32 = 32;
pub const crypto_aead_chacha20poly1305_IETF_NSECBYTES: u32 = 0;
pub const crypto_aead_chacha20poly1305_IETF_NPUBBYTES: u32 = 12;
pub const crypto_aead_chacha20poly1305_IETF_ABYTES: u32 = 16;
pub const crypto_aead_xchacha20poly1305_ietf_KEYBYTES: u32 = 32;
pub const crypto_aead_xchacha20poly1305_ietf_NSECBYTES: u32 = 0;
pub const crypto_aead_xchacha20poly1305_ietf_NPUBBYTES: u32 = 24;
pub const crypto_aead_xchacha20poly1305_ietf_ABYTES: u32 = 16;
pub const crypto_aead_xchacha20poly1305_IETF_KEYBYTES: u32 = 32;
pub const crypto_aead_xchacha20poly1305_IETF_NSECBYTES: u32 = 0;
pub const crypto_aead_xchacha20poly1305_IETF_NPUBBYTES: u32 = 24;
pub const crypto_aead_xchacha20poly1305_IETF_ABYTES: u32 = 16;
pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __STDC_NO_THREADS__: u32 = 1;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 27;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const _STDLIB_H: u32 = 1;
pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;
pub const WSTOPPED: u32 = 2;
pub const WEXITED: u32 = 4;
pub const WCONTINUED: u32 = 8;
pub const WNOWAIT: u32 = 16777216;
pub const __WNOTHREAD: u32 = 536870912;
pub const __WALL: u32 = 1073741824;
pub const __WCLONE: u32 = 2147483648;
pub const __W_CONTINUED: u32 = 65535;
pub const __WCOREFLAG: u32 = 128;
pub const __HAVE_FLOAT128: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128: u32 = 0;
pub const __HAVE_FLOAT64X: u32 = 1;
pub const __HAVE_FLOAT64X_LONG_DOUBLE: u32 = 1;
pub const __HAVE_FLOAT16: u32 = 0;
pub const __HAVE_FLOAT32: u32 = 1;
pub const __HAVE_FLOAT64: u32 = 1;
pub const __HAVE_FLOAT32X: u32 = 1;
pub const __HAVE_FLOAT128X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT16: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128X: u32 = 0;
pub const __HAVE_FLOATN_NOT_TYPEDEF: u32 = 0;
pub const __ldiv_t_defined: u32 = 1;
pub const __lldiv_t_defined: u32 = 1;
pub const RAND_MAX: u32 = 2147483647;
pub const EXIT_FAILURE: u32 = 1;
pub const EXIT_SUCCESS: u32 = 0;
pub const _SYS_TYPES_H: u32 = 1;
pub const __clock_t_defined: u32 = 1;
pub const __clockid_t_defined: u32 = 1;
pub const __time_t_defined: u32 = 1;
pub const __timer_t_defined: u32 = 1;
pub const __BIT_TYPES_DEFINED__: u32 = 1;
pub const _ENDIAN_H: u32 = 1;
pub const __LITTLE_ENDIAN: u32 = 1234;
pub const __BIG_ENDIAN: u32 = 4321;
pub const __PDP_ENDIAN: u32 = 3412;
pub const __BYTE_ORDER: u32 = 1234;
pub const __FLOAT_WORD_ORDER: u32 = 1234;
pub const LITTLE_ENDIAN: u32 = 1234;
pub const BIG_ENDIAN: u32 = 4321;
pub const PDP_ENDIAN: u32 = 3412;
pub const BYTE_ORDER: u32 = 1234;
pub const _BITS_BYTESWAP_H: u32 = 1;
pub const _BITS_UINTN_IDENTITY_H: u32 = 1;
pub const _SYS_SELECT_H: u32 = 1;
pub const __FD_ZERO_STOS: &'static [u8; 6usize] = b"stosq\0";
pub const __sigset_t_defined: u32 = 1;
pub const __timeval_defined: u32 = 1;
pub const __timespec_defined: u32 = 1;
pub const FD_SETSIZE: u32 = 1024;
pub const _SYS_SYSMACROS_H: u32 = 1;
pub const _BITS_SYSMACROS_H: u32 = 1;
pub const _BITS_PTHREADTYPES_COMMON_H: u32 = 1;
pub const _THREAD_SHARED_TYPES_H: u32 = 1;
pub const _BITS_PTHREADTYPES_ARCH_H: u32 = 1;
pub const __SIZEOF_PTHREAD_MUTEX_T: u32 = 40;
pub const __SIZEOF_PTHREAD_ATTR_T: u32 = 56;
pub const __SIZEOF_PTHREAD_RWLOCK_T: u32 = 56;
pub const __SIZEOF_PTHREAD_BARRIER_T: u32 = 32;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_COND_T: u32 = 48;
pub const __SIZEOF_PTHREAD_CONDATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: u32 = 8;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: u32 = 4;
pub const __PTHREAD_MUTEX_LOCK_ELISION: u32 = 1;
pub const __PTHREAD_MUTEX_NUSERS_AFTER_KIND: u32 = 0;
pub const __PTHREAD_MUTEX_USE_UNION: u32 = 0;
pub const __PTHREAD_RWLOCK_INT_FLAGS_SHARED: u32 = 1;
pub const __PTHREAD_MUTEX_HAVE_PREV: u32 = 1;
pub const __have_pthread_attr_t: u32 = 1;
pub const _ALLOCA_H: u32 = 1;
pub const crypto_hash_sha512_BYTES: u32 = 64;
pub const crypto_auth_hmacsha512_BYTES: u32 = 64;
pub const crypto_auth_hmacsha512_KEYBYTES: u32 = 32;
pub const crypto_auth_hmacsha512256_BYTES: u32 = 32;
pub const crypto_auth_hmacsha512256_KEYBYTES: u32 = 32;
pub const crypto_auth_BYTES: u32 = 32;
pub const crypto_auth_KEYBYTES: u32 = 32;
pub const crypto_auth_PRIMITIVE: &'static [u8; 14usize] = b"hmacsha512256\0";
pub const crypto_hash_sha256_BYTES: u32 = 32;
pub const crypto_auth_hmacsha256_BYTES: u32 = 32;
pub const crypto_auth_hmacsha256_KEYBYTES: u32 = 32;
pub const crypto_stream_xsalsa20_KEYBYTES: u32 = 32;
pub const crypto_stream_xsalsa20_NONCEBYTES: u32 = 24;
pub const crypto_box_curve25519xsalsa20poly1305_SEEDBYTES: u32 = 32;
pub const crypto_box_curve25519xsalsa20poly1305_PUBLICKEYBYTES: u32 = 32;
pub const crypto_box_curve25519xsalsa20poly1305_SECRETKEYBYTES: u32 = 32;
pub const crypto_box_curve25519xsalsa20poly1305_BEFORENMBYTES: u32 = 32;
pub const crypto_box_curve25519xsalsa20poly1305_NONCEBYTES: u32 = 24;
pub const crypto_box_curve25519xsalsa20poly1305_MACBYTES: u32 = 16;
pub const crypto_box_curve25519xsalsa20poly1305_BOXZEROBYTES: u32 = 16;
pub const crypto_box_curve25519xsalsa20poly1305_ZEROBYTES: u32 = 32;
pub const crypto_box_SEEDBYTES: u32 = 32;
pub const crypto_box_PUBLICKEYBYTES: u32 = 32;
pub const crypto_box_SECRETKEYBYTES: u32 = 32;
pub const crypto_box_NONCEBYTES: u32 = 24;
pub const crypto_box_MACBYTES: u32 = 16;
pub const crypto_box_PRIMITIVE: &'static [u8; 27usize] = b"curve25519xsalsa20poly1305\0";
pub const crypto_box_BEFORENMBYTES: u32 = 32;
pub const crypto_box_SEALBYTES: u32 = 48;
pub const crypto_box_ZEROBYTES: u32 = 32;
pub const crypto_box_BOXZEROBYTES: u32 = 16;
pub const crypto_core_hsalsa20_OUTPUTBYTES: u32 = 32;
pub const crypto_core_hsalsa20_INPUTBYTES: u32 = 16;
pub const crypto_core_hsalsa20_KEYBYTES: u32 = 32;
pub const crypto_core_hsalsa20_CONSTBYTES: u32 = 16;
pub const crypto_core_hchacha20_OUTPUTBYTES: u32 = 32;
pub const crypto_core_hchacha20_INPUTBYTES: u32 = 16;
pub const crypto_core_hchacha20_KEYBYTES: u32 = 32;
pub const crypto_core_hchacha20_CONSTBYTES: u32 = 16;
pub const crypto_core_salsa20_OUTPUTBYTES: u32 = 64;
pub const crypto_core_salsa20_INPUTBYTES: u32 = 16;
pub const crypto_core_salsa20_KEYBYTES: u32 = 32;
pub const crypto_core_salsa20_CONSTBYTES: u32 = 16;
pub const crypto_core_salsa2012_OUTPUTBYTES: u32 = 64;
pub const crypto_core_salsa2012_INPUTBYTES: u32 = 16;
pub const crypto_core_salsa2012_KEYBYTES: u32 = 32;
pub const crypto_core_salsa2012_CONSTBYTES: u32 = 16;
pub const crypto_core_salsa208_OUTPUTBYTES: u32 = 64;
pub const crypto_core_salsa208_INPUTBYTES: u32 = 16;
pub const crypto_core_salsa208_KEYBYTES: u32 = 32;
pub const crypto_core_salsa208_CONSTBYTES: u32 = 16;
pub const crypto_generichash_blake2b_BYTES_MIN: u32 = 16;
pub const crypto_generichash_blake2b_BYTES_MAX: u32 = 64;
pub const crypto_generichash_blake2b_BYTES: u32 = 32;
pub const crypto_generichash_blake2b_KEYBYTES_MIN: u32 = 16;
pub const crypto_generichash_blake2b_KEYBYTES_MAX: u32 = 64;
pub const crypto_generichash_blake2b_KEYBYTES: u32 = 32;
pub const crypto_generichash_blake2b_SALTBYTES: u32 = 16;
pub const crypto_generichash_blake2b_PERSONALBYTES: u32 = 16;
pub const crypto_generichash_BYTES_MIN: u32 = 16;
pub const crypto_generichash_BYTES_MAX: u32 = 64;
pub const crypto_generichash_BYTES: u32 = 32;
pub const crypto_generichash_KEYBYTES_MIN: u32 = 16;
pub const crypto_generichash_KEYBYTES_MAX: u32 = 64;
pub const crypto_generichash_KEYBYTES: u32 = 32;
pub const crypto_generichash_PRIMITIVE: &'static [u8; 8usize] = b"blake2b\0";
pub const crypto_hash_BYTES: u32 = 64;
pub const crypto_hash_PRIMITIVE: &'static [u8; 7usize] = b"sha512\0";
pub const crypto_kdf_blake2b_BYTES_MIN: u32 = 16;
pub const crypto_kdf_blake2b_BYTES_MAX: u32 = 64;
pub const crypto_kdf_blake2b_CONTEXTBYTES: u32 = 8;
pub const crypto_kdf_blake2b_KEYBYTES: u32 = 32;
pub const crypto_kdf_BYTES_MIN: u32 = 16;
pub const crypto_kdf_BYTES_MAX: u32 = 64;
pub const crypto_kdf_CONTEXTBYTES: u32 = 8;
pub const crypto_kdf_KEYBYTES: u32 = 32;
pub const crypto_kdf_PRIMITIVE: &'static [u8; 8usize] = b"blake2b\0";
pub const crypto_kx_PUBLICKEYBYTES: u32 = 32;
pub const crypto_kx_SECRETKEYBYTES: u32 = 32;
pub const crypto_kx_SEEDBYTES: u32 = 32;
pub const crypto_kx_SESSIONKEYBYTES: u32 = 32;
pub const crypto_kx_PRIMITIVE: &'static [u8; 14usize] = b"x25519blake2b\0";
pub const _STDIO_H: u32 = 1;
pub const ____FILE_defined: u32 = 1;
pub const __FILE_defined: u32 = 1;
pub const _BITS_LIBIO_H: u32 = 1;
pub const _BITS_G_CONFIG_H: u32 = 1;
pub const ____mbstate_t_defined: u32 = 1;
pub const _G_HAVE_MMAP: u32 = 1;
pub const _G_HAVE_MREMAP: u32 = 1;
pub const _G_IO_IO_FILE_VERSION: u32 = 131073;
pub const _G_BUFSIZ: u32 = 8192;
pub const _IO_BUFSIZ: u32 = 8192;
pub const __GNUC_VA_LIST: u32 = 1;
pub const _IO_UNIFIED_JUMPTABLES: u32 = 1;
pub const EOF: i32 = -1;
pub const _IOS_INPUT: u32 = 1;
pub const _IOS_OUTPUT: u32 = 2;
pub const _IOS_ATEND: u32 = 4;
pub const _IOS_APPEND: u32 = 8;
pub const _IOS_TRUNC: u32 = 16;
pub const _IOS_NOCREATE: u32 = 32;
pub const _IOS_NOREPLACE: u32 = 64;
pub const _IOS_BIN: u32 = 128;
pub const _IO_MAGIC: u32 = 4222418944;
pub const _OLD_STDIO_MAGIC: u32 = 4206624768;
pub const _IO_MAGIC_MASK: u32 = 4294901760;
pub const _IO_USER_BUF: u32 = 1;
pub const _IO_UNBUFFERED: u32 = 2;
pub const _IO_NO_READS: u32 = 4;
pub const _IO_NO_WRITES: u32 = 8;
pub const _IO_EOF_SEEN: u32 = 16;
pub const _IO_ERR_SEEN: u32 = 32;
pub const _IO_DELETE_DONT_CLOSE: u32 = 64;
pub const _IO_LINKED: u32 = 128;
pub const _IO_IN_BACKUP: u32 = 256;
pub const _IO_LINE_BUF: u32 = 512;
pub const _IO_TIED_PUT_GET: u32 = 1024;
pub const _IO_CURRENTLY_PUTTING: u32 = 2048;
pub const _IO_IS_APPENDING: u32 = 4096;
pub const _IO_IS_FILEBUF: u32 = 8192;
pub const _IO_BAD_SEEN: u32 = 16384;
pub const _IO_USER_LOCK: u32 = 32768;
pub const _IO_FLAGS2_MMAP: u32 = 1;
pub const _IO_FLAGS2_NOTCANCEL: u32 = 2;
pub const _IO_FLAGS2_USER_WBUF: u32 = 8;
pub const _IO_SKIPWS: u32 = 1;
pub const _IO_LEFT: u32 = 2;
pub const _IO_RIGHT: u32 = 4;
pub const _IO_INTERNAL: u32 = 8;
pub const _IO_DEC: u32 = 16;
pub const _IO_OCT: u32 = 32;
pub const _IO_HEX: u32 = 64;
pub const _IO_SHOWBASE: u32 = 128;
pub const _IO_SHOWPOINT: u32 = 256;
pub const _IO_UPPERCASE: u32 = 512;
pub const _IO_SHOWPOS: u32 = 1024;
pub const _IO_SCIENTIFIC: u32 = 2048;
pub const _IO_FIXED: u32 = 4096;
pub const _IO_UNITBUF: u32 = 8192;
pub const _IO_STDIO: u32 = 16384;
pub const _IO_DONT_CLOSE: u32 = 32768;
pub const _IO_BOOLALPHA: u32 = 65536;
pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 1;
pub const _IONBF: u32 = 2;
pub const BUFSIZ: u32 = 8192;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const P_tmpdir: &'static [u8; 5usize] = b"/tmp\0";
pub const _BITS_STDIO_LIM_H: u32 = 1;
pub const L_tmpnam: u32 = 20;
pub const TMP_MAX: u32 = 238328;
pub const FILENAME_MAX: u32 = 4096;
pub const L_ctermid: u32 = 9;
pub const FOPEN_MAX: u32 = 16;
pub const crypto_onetimeauth_poly1305_BYTES: u32 = 16;
pub const crypto_onetimeauth_poly1305_KEYBYTES: u32 = 32;
pub const crypto_onetimeauth_BYTES: u32 = 16;
pub const crypto_onetimeauth_KEYBYTES: u32 = 32;
pub const crypto_onetimeauth_PRIMITIVE: &'static [u8; 9usize] = b"poly1305\0";
pub const _LIBC_LIMITS_H_: u32 = 1;
pub const MB_LEN_MAX: u32 = 16;
pub const _BITS_POSIX1_LIM_H: u32 = 1;
pub const _POSIX_AIO_LISTIO_MAX: u32 = 2;
pub const _POSIX_AIO_MAX: u32 = 1;
pub const _POSIX_ARG_MAX: u32 = 4096;
pub const _POSIX_CHILD_MAX: u32 = 25;
pub const _POSIX_DELAYTIMER_MAX: u32 = 32;
pub const _POSIX_HOST_NAME_MAX: u32 = 255;
pub const _POSIX_LINK_MAX: u32 = 8;
pub const _POSIX_LOGIN_NAME_MAX: u32 = 9;
pub const _POSIX_MAX_CANON: u32 = 255;
pub const _POSIX_MAX_INPUT: u32 = 255;
pub const _POSIX_MQ_OPEN_MAX: u32 = 8;
pub const _POSIX_MQ_PRIO_MAX: u32 = 32;
pub const _POSIX_NAME_MAX: u32 = 14;
pub const _POSIX_NGROUPS_MAX: u32 = 8;
pub const _POSIX_OPEN_MAX: u32 = 20;
pub const _POSIX_PATH_MAX: u32 = 256;
pub const _POSIX_PIPE_BUF: u32 = 512;
pub const _POSIX_RE_DUP_MAX: u32 = 255;
pub const _POSIX_RTSIG_MAX: u32 = 8;
pub const _POSIX_SEM_NSEMS_MAX: u32 = 256;
pub const _POSIX_SEM_VALUE_MAX: u32 = 32767;
pub const _POSIX_SIGQUEUE_MAX: u32 = 32;
pub const _POSIX_SSIZE_MAX: u32 = 32767;
pub const _POSIX_STREAM_MAX: u32 = 8;
pub const _POSIX_SYMLINK_MAX: u32 = 255;
pub const _POSIX_SYMLOOP_MAX: u32 = 8;
pub const _POSIX_TIMER_MAX: u32 = 32;
pub const _POSIX_TTY_NAME_MAX: u32 = 9;
pub const _POSIX_TZNAME_MAX: u32 = 6;
pub const _POSIX_CLOCKRES_MIN: u32 = 20000000;
pub const NR_OPEN: u32 = 1024;
pub const NGROUPS_MAX: u32 = 65536;
pub const ARG_MAX: u32 = 131072;
pub const LINK_MAX: u32 = 127;
pub const MAX_CANON: u32 = 255;
pub const MAX_INPUT: u32 = 255;
pub const NAME_MAX: u32 = 255;
pub const PATH_MAX: u32 = 4096;
pub const PIPE_BUF: u32 = 4096;
pub const XATTR_NAME_MAX: u32 = 255;
pub const XATTR_SIZE_MAX: u32 = 65536;
pub const XATTR_LIST_MAX: u32 = 65536;
pub const RTSIG_MAX: u32 = 32;
pub const _POSIX_THREAD_KEYS_MAX: u32 = 128;
pub const PTHREAD_KEYS_MAX: u32 = 1024;
pub const _POSIX_THREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
pub const PTHREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
pub const _POSIX_THREAD_THREADS_MAX: u32 = 64;
pub const AIO_PRIO_DELTA_MAX: u32 = 20;
pub const PTHREAD_STACK_MIN: u32 = 16384;
pub const DELAYTIMER_MAX: u32 = 2147483647;
pub const TTY_NAME_MAX: u32 = 32;
pub const LOGIN_NAME_MAX: u32 = 256;
pub const HOST_NAME_MAX: u32 = 64;
pub const MQ_PRIO_MAX: u32 = 32768;
pub const SEM_VALUE_MAX: u32 = 2147483647;
pub const _BITS_POSIX2_LIM_H: u32 = 1;
pub const _POSIX2_BC_BASE_MAX: u32 = 99;
pub const _POSIX2_BC_DIM_MAX: u32 = 2048;
pub const _POSIX2_BC_SCALE_MAX: u32 = 99;
pub const _POSIX2_BC_STRING_MAX: u32 = 1000;
pub const _POSIX2_COLL_WEIGHTS_MAX: u32 = 2;
pub const _POSIX2_EXPR_NEST_MAX: u32 = 32;
pub const _POSIX2_LINE_MAX: u32 = 2048;
pub const _POSIX2_RE_DUP_MAX: u32 = 255;
pub const _POSIX2_CHARCLASS_NAME_MAX: u32 = 14;
pub const BC_BASE_MAX: u32 = 99;
pub const BC_DIM_MAX: u32 = 2048;
pub const BC_SCALE_MAX: u32 = 99;
pub const BC_STRING_MAX: u32 = 1000;
pub const COLL_WEIGHTS_MAX: u32 = 255;
pub const EXPR_NEST_MAX: u32 = 32;
pub const LINE_MAX: u32 = 2048;
pub const CHARCLASS_NAME_MAX: u32 = 2048;
pub const RE_DUP_MAX: u32 = 32767;
pub const crypto_pwhash_argon2i_ALG_ARGON2I13: u32 = 1;
pub const crypto_pwhash_argon2i_BYTES_MIN: u32 = 16;
pub const crypto_pwhash_argon2i_PASSWD_MIN: u32 = 0;
pub const crypto_pwhash_argon2i_PASSWD_MAX: u32 = 4294967295;
pub const crypto_pwhash_argon2i_SALTBYTES: u32 = 16;
pub const crypto_pwhash_argon2i_STRBYTES: u32 = 128;
pub const crypto_pwhash_argon2i_STRPREFIX: &'static [u8; 10usize] = b"$argon2i$\0";
pub const crypto_pwhash_argon2i_OPSLIMIT_MIN: u32 = 3;
pub const crypto_pwhash_argon2i_OPSLIMIT_MAX: u32 = 4294967295;
pub const crypto_pwhash_argon2i_MEMLIMIT_MIN: u32 = 8192;
pub const crypto_pwhash_argon2i_OPSLIMIT_INTERACTIVE: u32 = 4;
pub const crypto_pwhash_argon2i_MEMLIMIT_INTERACTIVE: u32 = 33554432;
pub const crypto_pwhash_argon2i_OPSLIMIT_MODERATE: u32 = 6;
pub const crypto_pwhash_argon2i_MEMLIMIT_MODERATE: u32 = 134217728;
pub const crypto_pwhash_argon2i_OPSLIMIT_SENSITIVE: u32 = 8;
pub const crypto_pwhash_argon2i_MEMLIMIT_SENSITIVE: u32 = 536870912;
pub const crypto_pwhash_argon2id_ALG_ARGON2ID13: u32 = 2;
pub const crypto_pwhash_argon2id_BYTES_MIN: u32 = 16;
pub const crypto_pwhash_argon2id_PASSWD_MIN: u32 = 0;
pub const crypto_pwhash_argon2id_PASSWD_MAX: u32 = 4294967295;
pub const crypto_pwhash_argon2id_SALTBYTES: u32 = 16;
pub const crypto_pwhash_argon2id_STRBYTES: u32 = 128;
pub const crypto_pwhash_argon2id_STRPREFIX: &'static [u8; 11usize] = b"$argon2id$\0";
pub const crypto_pwhash_argon2id_OPSLIMIT_MIN: u32 = 1;
pub const crypto_pwhash_argon2id_OPSLIMIT_MAX: u32 = 4294967295;
pub const crypto_pwhash_argon2id_MEMLIMIT_MIN: u32 = 8192;
pub const crypto_pwhash_argon2id_OPSLIMIT_INTERACTIVE: u32 = 2;
pub const crypto_pwhash_argon2id_MEMLIMIT_INTERACTIVE: u32 = 67108864;
pub const crypto_pwhash_argon2id_OPSLIMIT_MODERATE: u32 = 3;
pub const crypto_pwhash_argon2id_MEMLIMIT_MODERATE: u32 = 268435456;
pub const crypto_pwhash_argon2id_OPSLIMIT_SENSITIVE: u32 = 4;
pub const crypto_pwhash_argon2id_MEMLIMIT_SENSITIVE: u32 = 1073741824;
pub const crypto_pwhash_ALG_ARGON2I13: u32 = 1;
pub const crypto_pwhash_ALG_ARGON2ID13: u32 = 2;
pub const crypto_pwhash_ALG_DEFAULT: u32 = 2;
pub const crypto_pwhash_BYTES_MIN: u32 = 16;
pub const crypto_pwhash_PASSWD_MIN: u32 = 0;
pub const crypto_pwhash_PASSWD_MAX: u32 = 4294967295;
pub const crypto_pwhash_SALTBYTES: u32 = 16;
pub const crypto_pwhash_STRBYTES: u32 = 128;
pub const crypto_pwhash_STRPREFIX: &'static [u8; 11usize] = b"$argon2id$\0";
pub const crypto_pwhash_OPSLIMIT_MIN: u32 = 1;
pub const crypto_pwhash_OPSLIMIT_MAX: u32 = 4294967295;
pub const crypto_pwhash_MEMLIMIT_MIN: u32 = 8192;
pub const crypto_pwhash_OPSLIMIT_INTERACTIVE: u32 = 2;
pub const crypto_pwhash_MEMLIMIT_INTERACTIVE: u32 = 67108864;
pub const crypto_pwhash_OPSLIMIT_MODERATE: u32 = 3;
pub const crypto_pwhash_MEMLIMIT_MODERATE: u32 = 268435456;
pub const crypto_pwhash_OPSLIMIT_SENSITIVE: u32 = 4;
pub const crypto_pwhash_MEMLIMIT_SENSITIVE: u32 = 1073741824;
pub const crypto_pwhash_PRIMITIVE: &'static [u8; 8usize] = b"argon2i\0";
pub const crypto_scalarmult_curve25519_BYTES: u32 = 32;
pub const crypto_scalarmult_curve25519_SCALARBYTES: u32 = 32;
pub const crypto_scalarmult_BYTES: u32 = 32;
pub const crypto_scalarmult_SCALARBYTES: u32 = 32;
pub const crypto_scalarmult_PRIMITIVE: &'static [u8; 11usize] = b"curve25519\0";
pub const crypto_secretbox_xsalsa20poly1305_KEYBYTES: u32 = 32;
pub const crypto_secretbox_xsalsa20poly1305_NONCEBYTES: u32 = 24;
pub const crypto_secretbox_xsalsa20poly1305_MACBYTES: u32 = 16;
pub const crypto_secretbox_xsalsa20poly1305_BOXZEROBYTES: u32 = 16;
pub const crypto_secretbox_xsalsa20poly1305_ZEROBYTES: u32 = 32;
pub const crypto_secretbox_KEYBYTES: u32 = 32;
pub const crypto_secretbox_NONCEBYTES: u32 = 24;
pub const crypto_secretbox_MACBYTES: u32 = 16;
pub const crypto_secretbox_PRIMITIVE: &'static [u8; 17usize] = b"xsalsa20poly1305\0";
pub const crypto_secretbox_ZEROBYTES: u32 = 32;
pub const crypto_secretbox_BOXZEROBYTES: u32 = 16;
pub const crypto_stream_chacha20_KEYBYTES: u32 = 32;
pub const crypto_stream_chacha20_NONCEBYTES: u32 = 8;
pub const crypto_stream_chacha20_ietf_KEYBYTES: u32 = 32;
pub const crypto_stream_chacha20_ietf_NONCEBYTES: u32 = 12;
pub const crypto_stream_chacha20_IETF_KEYBYTES: u32 = 32;
pub const crypto_stream_chacha20_IETF_NONCEBYTES: u32 = 12;
pub const crypto_secretstream_xchacha20poly1305_ABYTES: u32 = 17;
pub const crypto_secretstream_xchacha20poly1305_HEADERBYTES: u32 = 24;
pub const crypto_secretstream_xchacha20poly1305_KEYBYTES: u32 = 32;
pub const crypto_secretstream_xchacha20poly1305_TAG_MESSAGE: u32 = 0;
pub const crypto_secretstream_xchacha20poly1305_TAG_PUSH: u32 = 1;
pub const crypto_secretstream_xchacha20poly1305_TAG_REKEY: u32 = 2;
pub const crypto_secretstream_xchacha20poly1305_TAG_FINAL: u32 = 3;
pub const crypto_shorthash_siphash24_BYTES: u32 = 8;
pub const crypto_shorthash_siphash24_KEYBYTES: u32 = 16;
pub const crypto_shorthash_siphashx24_BYTES: u32 = 16;
pub const crypto_shorthash_siphashx24_KEYBYTES: u32 = 16;
pub const crypto_shorthash_BYTES: u32 = 8;
pub const crypto_shorthash_KEYBYTES: u32 = 16;
pub const crypto_shorthash_PRIMITIVE: &'static [u8; 10usize] = b"siphash24\0";
pub const crypto_sign_ed25519_BYTES: u32 = 64;
pub const crypto_sign_ed25519_SEEDBYTES: u32 = 32;
pub const crypto_sign_ed25519_PUBLICKEYBYTES: u32 = 32;
pub const crypto_sign_ed25519_SECRETKEYBYTES: u32 = 64;
pub const crypto_sign_BYTES: u32 = 64;
pub const crypto_sign_SEEDBYTES: u32 = 32;
pub const crypto_sign_PUBLICKEYBYTES: u32 = 32;
pub const crypto_sign_SECRETKEYBYTES: u32 = 64;
pub const crypto_sign_PRIMITIVE: &'static [u8; 8usize] = b"ed25519\0";
pub const crypto_stream_KEYBYTES: u32 = 32;
pub const crypto_stream_NONCEBYTES: u32 = 24;
pub const crypto_stream_PRIMITIVE: &'static [u8; 9usize] = b"xsalsa20\0";
pub const crypto_stream_salsa20_KEYBYTES: u32 = 32;
pub const crypto_stream_salsa20_NONCEBYTES: u32 = 8;
pub const crypto_verify_16_BYTES: u32 = 16;
pub const crypto_verify_32_BYTES: u32 = 32;
pub const crypto_verify_64_BYTES: u32 = 64;
pub const randombytes_SEEDBYTES: u32 = 32;
pub const sodium_base64_VARIANT_ORIGINAL: u32 = 1;
pub const sodium_base64_VARIANT_ORIGINAL_NO_PADDING: u32 = 3;
pub const sodium_base64_VARIANT_URLSAFE: u32 = 5;
pub const sodium_base64_VARIANT_URLSAFE_NO_PADDING: u32 = 7;
pub const crypto_stream_xchacha20_KEYBYTES: u32 = 32;
pub const crypto_stream_xchacha20_NONCEBYTES: u32 = 24;
pub const crypto_box_curve25519xchacha20poly1305_SEEDBYTES: u32 = 32;
pub const crypto_box_curve25519xchacha20poly1305_PUBLICKEYBYTES: u32 = 32;
pub const crypto_box_curve25519xchacha20poly1305_SECRETKEYBYTES: u32 = 32;
pub const crypto_box_curve25519xchacha20poly1305_BEFORENMBYTES: u32 = 32;
pub const crypto_box_curve25519xchacha20poly1305_NONCEBYTES: u32 = 24;
pub const crypto_box_curve25519xchacha20poly1305_MACBYTES: u32 = 16;
pub const crypto_box_curve25519xchacha20poly1305_SEALBYTES: u32 = 48;
pub const crypto_core_ed25519_BYTES: u32 = 32;
pub const crypto_core_ed25519_UNIFORMBYTES: u32 = 32;
pub const crypto_scalarmult_ed25519_BYTES: u32 = 32;
pub const crypto_scalarmult_ed25519_SCALARBYTES: u32 = 32;
pub const crypto_secretbox_xchacha20poly1305_KEYBYTES: u32 = 32;
pub const crypto_secretbox_xchacha20poly1305_NONCEBYTES: u32 = 24;
pub const crypto_secretbox_xchacha20poly1305_MACBYTES: u32 = 16;
pub const crypto_pwhash_scryptsalsa208sha256_BYTES_MIN: u32 = 16;
pub const crypto_pwhash_scryptsalsa208sha256_PASSWD_MIN: u32 = 0;
pub const crypto_pwhash_scryptsalsa208sha256_SALTBYTES: u32 = 32;
pub const crypto_pwhash_scryptsalsa208sha256_STRBYTES: u32 = 102;
pub const crypto_pwhash_scryptsalsa208sha256_STRPREFIX: &'static [u8; 4usize] = b"$7$\0";
pub const crypto_pwhash_scryptsalsa208sha256_OPSLIMIT_MIN: u32 = 32768;
pub const crypto_pwhash_scryptsalsa208sha256_OPSLIMIT_MAX: u32 = 4294967295;
pub const crypto_pwhash_scryptsalsa208sha256_MEMLIMIT_MIN: u32 = 16777216;
pub const crypto_pwhash_scryptsalsa208sha256_OPSLIMIT_INTERACTIVE: u32 = 524288;
pub const crypto_pwhash_scryptsalsa208sha256_MEMLIMIT_INTERACTIVE: u32 = 16777216;
pub const crypto_pwhash_scryptsalsa208sha256_OPSLIMIT_SENSITIVE: u32 = 33554432;
pub const crypto_pwhash_scryptsalsa208sha256_MEMLIMIT_SENSITIVE: u32 = 1073741824;
pub const crypto_stream_salsa2012_KEYBYTES: u32 = 32;
pub const crypto_stream_salsa2012_NONCEBYTES: u32 = 8;
pub const crypto_stream_salsa208_KEYBYTES: u32 = 32;
pub const crypto_stream_salsa208_NONCEBYTES: u32 = 8;
extern "C" {
    pub fn sodium_version_string() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn sodium_library_version_major() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_library_version_minor() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_library_minimal() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_init() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_set_misuse_handler(
        handler: ::std::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_misuse();
}
pub type wchar_t = ::std::os::raw::c_int;
extern "C" {
    pub fn crypto_aead_aes256gcm_is_available() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_nsecbytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_npubbytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_abytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_messagebytes_max() -> usize;
}
pub type crypto_aead_aes256gcm_state = [::std::os::raw::c_uchar; 512usize];
extern "C" {
    pub fn crypto_aead_aes256gcm_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_encrypt(
        c: *mut ::std::os::raw::c_uchar,
        clen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_decrypt(
        m: *mut ::std::os::raw::c_uchar,
        mlen_p: *mut ::std::os::raw::c_ulonglong,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_encrypt_detached(
        c: *mut ::std::os::raw::c_uchar,
        mac: *mut ::std::os::raw::c_uchar,
        maclen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_decrypt_detached(
        m: *mut ::std::os::raw::c_uchar,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        mac: *const ::std::os::raw::c_uchar,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_beforenm(
        ctx_: *mut crypto_aead_aes256gcm_state,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_encrypt_afternm(
        c: *mut ::std::os::raw::c_uchar,
        clen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        ctx_: *const crypto_aead_aes256gcm_state,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_decrypt_afternm(
        m: *mut ::std::os::raw::c_uchar,
        mlen_p: *mut ::std::os::raw::c_ulonglong,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        ctx_: *const crypto_aead_aes256gcm_state,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_encrypt_detached_afternm(
        c: *mut ::std::os::raw::c_uchar,
        mac: *mut ::std::os::raw::c_uchar,
        maclen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        ctx_: *const crypto_aead_aes256gcm_state,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_decrypt_detached_afternm(
        m: *mut ::std::os::raw::c_uchar,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        mac: *const ::std::os::raw::c_uchar,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        ctx_: *const crypto_aead_aes256gcm_state,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_nsecbytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_npubbytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_abytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_encrypt(
        c: *mut ::std::os::raw::c_uchar,
        clen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_decrypt(
        m: *mut ::std::os::raw::c_uchar,
        mlen_p: *mut ::std::os::raw::c_ulonglong,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_encrypt_detached(
        c: *mut ::std::os::raw::c_uchar,
        mac: *mut ::std::os::raw::c_uchar,
        maclen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_decrypt_detached(
        m: *mut ::std::os::raw::c_uchar,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        mac: *const ::std::os::raw::c_uchar,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_nsecbytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_npubbytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_abytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_encrypt(
        c: *mut ::std::os::raw::c_uchar,
        clen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_decrypt(
        m: *mut ::std::os::raw::c_uchar,
        mlen_p: *mut ::std::os::raw::c_ulonglong,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_encrypt_detached(
        c: *mut ::std::os::raw::c_uchar,
        mac: *mut ::std::os::raw::c_uchar,
        maclen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_decrypt_detached(
        m: *mut ::std::os::raw::c_uchar,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        mac: *const ::std::os::raw::c_uchar,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_nsecbytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_npubbytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_abytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_encrypt(
        c: *mut ::std::os::raw::c_uchar,
        clen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_decrypt(
        m: *mut ::std::os::raw::c_uchar,
        mlen_p: *mut ::std::os::raw::c_ulonglong,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_encrypt_detached(
        c: *mut ::std::os::raw::c_uchar,
        mac: *mut ::std::os::raw::c_uchar,
        maclen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_decrypt_detached(
        m: *mut ::std::os::raw::c_uchar,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        mac: *const ::std::os::raw::c_uchar,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_keygen(k: *mut ::std::os::raw::c_uchar);
}
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__fsid_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__fsid_t>())).__val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__fsid_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_long;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct div_t {
    pub quot: ::std::os::raw::c_int,
    pub rem: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_div_t() {
    assert_eq!(
        ::std::mem::size_of::<div_t>(),
        8usize,
        concat!("Size of: ", stringify!(div_t))
    );
    assert_eq!(
        ::std::mem::align_of::<div_t>(),
        4usize,
        concat!("Alignment of ", stringify!(div_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<div_t>())).quot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(div_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<div_t>())).rem as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(div_t),
            "::",
            stringify!(rem)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ldiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_ldiv_t() {
    assert_eq!(
        ::std::mem::size_of::<ldiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(ldiv_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ldiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ldiv_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ldiv_t>())).quot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ldiv_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ldiv_t>())).rem as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ldiv_t),
            "::",
            stringify!(rem)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lldiv_t {
    pub quot: ::std::os::raw::c_longlong,
    pub rem: ::std::os::raw::c_longlong,
}
#[test]
fn bindgen_test_layout_lldiv_t() {
    assert_eq!(
        ::std::mem::size_of::<lldiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(lldiv_t))
    );
    assert_eq!(
        ::std::mem::align_of::<lldiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(lldiv_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<lldiv_t>())).quot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(lldiv_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<lldiv_t>())).rem as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(lldiv_t),
            "::",
            stringify!(rem)
        )
    );
}
extern "C" {
    pub fn __ctype_get_mb_cur_max() -> usize;
}
extern "C" {
    pub fn atof(__nptr: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn atoi(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atol(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn atoll(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtod(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn strtof(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f32;
}
extern "C" {
    pub fn strtold(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn strtol(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn strtoul(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strtoq(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtouq(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn strtoll(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoull(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn l64a(__n: ::std::os::raw::c_long) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn a64l(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type id_t = __id_t;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type timer_t = __timer_t;
pub type ulong = ::std::os::raw::c_ulong;
pub type ushort = ::std::os::raw::c_ushort;
pub type uint = ::std::os::raw::c_uint;
pub type u_int8_t = ::std::os::raw::c_uchar;
pub type u_int16_t = ::std::os::raw::c_ushort;
pub type u_int32_t = ::std::os::raw::c_uint;
pub type u_int64_t = ::std::os::raw::c_ulong;
pub type register_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sigset_t {
    pub __val: [::std::os::raw::c_ulong; 16usize],
}
#[test]
fn bindgen_test_layout___sigset_t() {
    assert_eq!(
        ::std::mem::size_of::<__sigset_t>(),
        128usize,
        concat!("Size of: ", stringify!(__sigset_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__sigset_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__sigset_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sigset_t>())).__val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigset_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type sigset_t = __sigset_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[test]
fn bindgen_test_layout_timeval() {
    assert_eq!(
        ::std::mem::size_of::<timeval>(),
        16usize,
        concat!("Size of: ", stringify!(timeval))
    );
    assert_eq!(
        ::std::mem::align_of::<timeval>(),
        8usize,
        concat!("Alignment of ", stringify!(timeval))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timeval>())).tv_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timeval>())).tv_usec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval),
            "::",
            stringify!(tv_usec)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[test]
fn bindgen_test_layout_timespec() {
    assert_eq!(
        ::std::mem::size_of::<timespec>(),
        16usize,
        concat!("Size of: ", stringify!(timespec))
    );
    assert_eq!(
        ::std::mem::align_of::<timespec>(),
        8usize,
        concat!("Alignment of ", stringify!(timespec))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timespec>())).tv_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timespec>())).tv_nsec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec),
            "::",
            stringify!(tv_nsec)
        )
    );
}
pub type suseconds_t = __suseconds_t;
pub type __fd_mask = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16usize],
}
#[test]
fn bindgen_test_layout_fd_set() {
    assert_eq!(
        ::std::mem::size_of::<fd_set>(),
        128usize,
        concat!("Size of: ", stringify!(fd_set))
    );
    assert_eq!(
        ::std::mem::align_of::<fd_set>(),
        8usize,
        concat!("Alignment of ", stringify!(fd_set))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fd_set>())).__fds_bits as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fd_set),
            "::",
            stringify!(__fds_bits)
        )
    );
}
pub type fd_mask = __fd_mask;
extern "C" {
    pub fn select(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pselect(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gnu_dev_major(__dev: __dev_t) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn gnu_dev_minor(__dev: __dev_t) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn gnu_dev_makedev(
        __major: ::std::os::raw::c_uint,
        __minor: ::std::os::raw::c_uint,
    ) -> __dev_t;
}
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::std::os::raw::c_uint,
    pub __writers: ::std::os::raw::c_uint,
    pub __wrphase_futex: ::std::os::raw::c_uint,
    pub __writers_futex: ::std::os::raw::c_uint,
    pub __pad3: ::std::os::raw::c_uint,
    pub __pad4: ::std::os::raw::c_uint,
    pub __cur_writer: ::std::os::raw::c_int,
    pub __shared: ::std::os::raw::c_int,
    pub __rwelision: ::std::os::raw::c_schar,
    pub __pad1: [::std::os::raw::c_uchar; 7usize],
    pub __pad2: ::std::os::raw::c_ulong,
    pub __flags: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___pthread_rwlock_arch_t() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_rwlock_arch_t>(),
        56usize,
        concat!("Size of: ", stringify!(__pthread_rwlock_arch_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_rwlock_arch_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_rwlock_arch_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__readers as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__readers)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__writers as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__writers)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__wrphase_futex as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__wrphase_futex)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__writers_futex as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__writers_futex)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__pad3 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__pad4 as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__cur_writer as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__cur_writer)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__shared as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__shared)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__rwelision as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__rwelision)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__pad1 as *const _ as usize },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__pad2 as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_rwlock_arch_t>())).__flags as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
#[test]
fn bindgen_test_layout___pthread_internal_list() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_internal_list>(),
        16usize,
        concat!("Size of: ", stringify!(__pthread_internal_list))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_internal_list>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_internal_list))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_internal_list>())).__prev as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_internal_list),
            "::",
            stringify!(__prev)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_internal_list>())).__next as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_internal_list),
            "::",
            stringify!(__next)
        )
    );
}
pub type __pthread_list_t = __pthread_internal_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: ::std::os::raw::c_int,
    pub __count: ::std::os::raw::c_uint,
    pub __owner: ::std::os::raw::c_int,
    pub __nusers: ::std::os::raw::c_uint,
    pub __kind: ::std::os::raw::c_int,
    pub __spins: ::std::os::raw::c_short,
    pub __elision: ::std::os::raw::c_short,
    pub __list: __pthread_list_t,
}
#[test]
fn bindgen_test_layout___pthread_mutex_s() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_mutex_s>(),
        40usize,
        concat!("Size of: ", stringify!(__pthread_mutex_s))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_mutex_s>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_mutex_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__lock as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__lock)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__count as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__owner as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__owner)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__nusers as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__nusers)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__kind as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__kind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__spins as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__spins)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__elision as *const _ as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__elision)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_mutex_s>())).__list as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__list)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s {
    pub __bindgen_anon_1: __pthread_cond_s__bindgen_ty_1,
    pub __bindgen_anon_2: __pthread_cond_s__bindgen_ty_2,
    pub __g_refs: [::std::os::raw::c_uint; 2usize],
    pub __g_size: [::std::os::raw::c_uint; 2usize],
    pub __g1_orig_size: ::std::os::raw::c_uint,
    pub __wrefs: ::std::os::raw::c_uint,
    pub __g_signals: [::std::os::raw::c_uint; 2usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_1 {
    pub __wseq: ::std::os::raw::c_ulonglong,
    pub __wseq32: __pthread_cond_s__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>())).__low
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(__low)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_1__bindgen_ty_1>())).__high
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(__high)
        )
    );
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_cond_s__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(__pthread_cond_s__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_cond_s__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_cond_s__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_1>())).__wseq as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1),
            "::",
            stringify!(__wseq)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_1>())).__wseq32 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_1),
            "::",
            stringify!(__wseq32)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_2 {
    pub __g1_start: ::std::os::raw::c_ulonglong,
    pub __g1_start32: __pthread_cond_s__bindgen_ty_2__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_2__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>())).__low
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(__low)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_2__bindgen_ty_1>())).__high
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(__high)
        )
    );
}
#[test]
fn bindgen_test_layout___pthread_cond_s__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_cond_s__bindgen_ty_2>(),
        8usize,
        concat!("Size of: ", stringify!(__pthread_cond_s__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_cond_s__bindgen_ty_2>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_cond_s__bindgen_ty_2))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_2>())).__g1_start as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2),
            "::",
            stringify!(__g1_start)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__pthread_cond_s__bindgen_ty_2>())).__g1_start32 as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s__bindgen_ty_2),
            "::",
            stringify!(__g1_start32)
        )
    );
}
#[test]
fn bindgen_test_layout___pthread_cond_s() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_cond_s>(),
        48usize,
        concat!("Size of: ", stringify!(__pthread_cond_s))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_cond_s>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_cond_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_cond_s>())).__g_refs as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g_refs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_cond_s>())).__g_size as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_cond_s>())).__g1_orig_size as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g1_orig_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_cond_s>())).__wrefs as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__wrefs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__pthread_cond_s>())).__g_signals as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g_signals)
        )
    );
}
pub type pthread_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutexattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_pthread_mutexattr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_mutexattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_mutexattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_mutexattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_mutexattr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_mutexattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutexattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_mutexattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutexattr_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_condattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_pthread_condattr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_condattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_condattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_condattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_condattr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_condattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_condattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_condattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_condattr_t),
            "::",
            stringify!(__align)
        )
    );
}
pub type pthread_key_t = ::std::os::raw::c_uint;
pub type pthread_once_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_attr_t {
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 7usize],
}
#[test]
fn bindgen_test_layout_pthread_attr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_attr_t>(),
        56usize,
        concat!("Size of: ", stringify!(pthread_attr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_attr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_attr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_attr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_attr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_attr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_attr_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::std::os::raw::c_char; 40usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 5usize],
}
#[test]
fn bindgen_test_layout_pthread_mutex_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_mutex_t>(),
        40usize,
        concat!("Size of: ", stringify!(pthread_mutex_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_mutex_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_mutex_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_mutex_t>())).__data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutex_t),
            "::",
            stringify!(__data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_mutex_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutex_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_mutex_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutex_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::std::os::raw::c_char; 48usize],
    pub __align: ::std::os::raw::c_longlong,
    _bindgen_union_align: [u64; 6usize],
}
#[test]
fn bindgen_test_layout_pthread_cond_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_cond_t>(),
        48usize,
        concat!("Size of: ", stringify!(pthread_cond_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_cond_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_cond_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_cond_t>())).__data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_cond_t),
            "::",
            stringify!(__data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_cond_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_cond_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_cond_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_cond_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 7usize],
}
#[test]
fn bindgen_test_layout_pthread_rwlock_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_rwlock_t>(),
        56usize,
        concat!("Size of: ", stringify!(pthread_rwlock_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_rwlock_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_rwlock_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_rwlock_t>())).__data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlock_t),
            "::",
            stringify!(__data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_rwlock_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlock_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_rwlock_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlock_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlockattr_t {
    pub __size: [::std::os::raw::c_char; 8usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_pthread_rwlockattr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_rwlockattr_t>(),
        8usize,
        concat!("Size of: ", stringify!(pthread_rwlockattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_rwlockattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_rwlockattr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_rwlockattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlockattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_rwlockattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlockattr_t),
            "::",
            stringify!(__align)
        )
    );
}
pub type pthread_spinlock_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrier_t {
    pub __size: [::std::os::raw::c_char; 32usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 4usize],
}
#[test]
fn bindgen_test_layout_pthread_barrier_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_barrier_t>(),
        32usize,
        concat!("Size of: ", stringify!(pthread_barrier_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_barrier_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_barrier_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_barrier_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrier_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_barrier_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrier_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrierattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_pthread_barrierattr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_barrierattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_barrierattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_barrierattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_barrierattr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_barrierattr_t>())).__size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrierattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<pthread_barrierattr_t>())).__align as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrierattr_t),
            "::",
            stringify!(__align)
        )
    );
}
extern "C" {
    pub fn random() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn srandom(__seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn initstate(
        __seed: ::std::os::raw::c_uint,
        __statebuf: *mut ::std::os::raw::c_char,
        __statelen: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn setstate(__statebuf: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct random_data {
    pub fptr: *mut i32,
    pub rptr: *mut i32,
    pub state: *mut i32,
    pub rand_type: ::std::os::raw::c_int,
    pub rand_deg: ::std::os::raw::c_int,
    pub rand_sep: ::std::os::raw::c_int,
    pub end_ptr: *mut i32,
}
#[test]
fn bindgen_test_layout_random_data() {
    assert_eq!(
        ::std::mem::size_of::<random_data>(),
        48usize,
        concat!("Size of: ", stringify!(random_data))
    );
    assert_eq!(
        ::std::mem::align_of::<random_data>(),
        8usize,
        concat!("Alignment of ", stringify!(random_data))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<random_data>())).fptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(fptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<random_data>())).rptr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<random_data>())).state as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<random_data>())).rand_type as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rand_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<random_data>())).rand_deg as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rand_deg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<random_data>())).rand_sep as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rand_sep)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<random_data>())).end_ptr as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(end_ptr)
        )
    );
}
extern "C" {
    pub fn random_r(__buf: *mut random_data, __result: *mut i32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srandom_r(
        __seed: ::std::os::raw::c_uint,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn initstate_r(
        __seed: ::std::os::raw::c_uint,
        __statebuf: *mut ::std::os::raw::c_char,
        __statelen: usize,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setstate_r(
        __statebuf: *mut ::std::os::raw::c_char,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rand() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand(__seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rand_r(__seed: *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn drand48() -> f64;
}
extern "C" {
    pub fn erand48(__xsubi: *mut ::std::os::raw::c_ushort) -> f64;
}
extern "C" {
    pub fn lrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn nrand48(__xsubi: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn mrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn jrand48(__xsubi: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn srand48(__seedval: ::std::os::raw::c_long);
}
extern "C" {
    pub fn seed48(__seed16v: *mut ::std::os::raw::c_ushort) -> *mut ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn lcong48(__param: *mut ::std::os::raw::c_ushort);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drand48_data {
    pub __x: [::std::os::raw::c_ushort; 3usize],
    pub __old_x: [::std::os::raw::c_ushort; 3usize],
    pub __c: ::std::os::raw::c_ushort,
    pub __init: ::std::os::raw::c_ushort,
    pub __a: ::std::os::raw::c_ulonglong,
}
#[test]
fn bindgen_test_layout_drand48_data() {
    assert_eq!(
        ::std::mem::size_of::<drand48_data>(),
        24usize,
        concat!("Size of: ", stringify!(drand48_data))
    );
    assert_eq!(
        ::std::mem::align_of::<drand48_data>(),
        8usize,
        concat!("Alignment of ", stringify!(drand48_data))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<drand48_data>())).__x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<drand48_data>())).__old_x as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__old_x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<drand48_data>())).__c as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__c)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<drand48_data>())).__init as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__init)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<drand48_data>())).__a as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__a)
        )
    );
}
extern "C" {
    pub fn drand48_r(__buffer: *mut drand48_data, __result: *mut f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn erand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nrand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jrand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand48_r(
        __seedval: ::std::os::raw::c_long,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn seed48_r(
        __seed16v: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lcong48_r(
        __param: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn malloc(__size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn calloc(__nmemb: usize, __size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn realloc(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn free(__ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn alloca(__size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn valloc(__size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn posix_memalign(
        __memptr: *mut *mut ::std::os::raw::c_void,
        __alignment: usize,
        __size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aligned_alloc(__alignment: usize, __size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn abort();
}
extern "C" {
    pub fn atexit(__func: ::std::option::Option<unsafe extern "C" fn()>) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn at_quick_exit(
        __func: ::std::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn on_exit(
        __func: ::std::option::Option<
            unsafe extern "C" fn(
                __status: ::std::os::raw::c_int,
                __arg: *mut ::std::os::raw::c_void,
            ),
        >,
        __arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn quick_exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _Exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn getenv(__name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn putenv(__string: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setenv(
        __name: *const ::std::os::raw::c_char,
        __value: *const ::std::os::raw::c_char,
        __replace: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unsetenv(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearenv() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mktemp(__template: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mkstemp(__template: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkstemps(
        __template: *mut ::std::os::raw::c_char,
        __suffixlen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkdtemp(__template: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn system(__command: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn realpath(
        __name: *const ::std::os::raw::c_char,
        __resolved: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
pub type __compar_fn_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: *const ::std::os::raw::c_void, arg2: *const ::std::os::raw::c_void)
        -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn bsearch(
        __key: *const ::std::os::raw::c_void,
        __base: *const ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_fn_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn qsort(
        __base: *mut ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_fn_t,
    );
}
extern "C" {
    pub fn abs(__x: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn labs(__x: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llabs(__x: ::std::os::raw::c_longlong) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn div(__numer: ::std::os::raw::c_int, __denom: ::std::os::raw::c_int) -> div_t;
}
extern "C" {
    pub fn ldiv(__numer: ::std::os::raw::c_long, __denom: ::std::os::raw::c_long) -> ldiv_t;
}
extern "C" {
    pub fn lldiv(
        __numer: ::std::os::raw::c_longlong,
        __denom: ::std::os::raw::c_longlong,
    ) -> lldiv_t;
}
extern "C" {
    pub fn ecvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn gcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qecvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qfcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qgcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ecvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fcvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qecvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qfcvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mblen(__s: *const ::std::os::raw::c_char, __n: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbtowc(
        __pwc: *mut wchar_t,
        __s: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wctomb(__s: *mut ::std::os::raw::c_char, __wchar: wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbstowcs(__pwcs: *mut wchar_t, __s: *const ::std::os::raw::c_char, __n: usize) -> usize;
}
extern "C" {
    pub fn wcstombs(__s: *mut ::std::os::raw::c_char, __pwcs: *const wchar_t, __n: usize) -> usize;
}
extern "C" {
    pub fn rpmatch(__response: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getsubopt(
        __optionp: *mut *mut ::std::os::raw::c_char,
        __tokens: *const *mut ::std::os::raw::c_char,
        __valuep: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getloadavg(__loadavg: *mut f64, __nelem: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_hash_sha512_state {
    pub state: [u64; 8usize],
    pub count: [u64; 2usize],
    pub buf: [u8; 128usize],
}
#[test]
fn bindgen_test_layout_crypto_hash_sha512_state() {
    assert_eq!(
        ::std::mem::size_of::<crypto_hash_sha512_state>(),
        208usize,
        concat!("Size of: ", stringify!(crypto_hash_sha512_state))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_hash_sha512_state>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_hash_sha512_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_hash_sha512_state>())).state as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_hash_sha512_state),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_hash_sha512_state>())).count as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_hash_sha512_state),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_hash_sha512_state>())).buf as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_hash_sha512_state),
            "::",
            stringify!(buf)
        )
    );
}
extern "C" {
    pub fn crypto_hash_sha512_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_hash_sha512_bytes() -> usize;
}
extern "C" {
    pub fn crypto_hash_sha512(
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_hash_sha512_init(state: *mut crypto_hash_sha512_state) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_hash_sha512_update(
        state: *mut crypto_hash_sha512_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_hash_sha512_final(
        state: *mut crypto_hash_sha512_state,
        out: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512_bytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_hmacsha512_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_hmacsha512(
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512_verify(
        h: *const ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_auth_hmacsha512_state {
    pub ictx: crypto_hash_sha512_state,
    pub octx: crypto_hash_sha512_state,
}
#[test]
fn bindgen_test_layout_crypto_auth_hmacsha512_state() {
    assert_eq!(
        ::std::mem::size_of::<crypto_auth_hmacsha512_state>(),
        416usize,
        concat!("Size of: ", stringify!(crypto_auth_hmacsha512_state))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_auth_hmacsha512_state>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_auth_hmacsha512_state))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_auth_hmacsha512_state>())).ictx as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_auth_hmacsha512_state),
            "::",
            stringify!(ictx)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_auth_hmacsha512_state>())).octx as *const _ as usize
        },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_auth_hmacsha512_state),
            "::",
            stringify!(octx)
        )
    );
}
extern "C" {
    pub fn crypto_auth_hmacsha512_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_hmacsha512_init(
        state: *mut crypto_auth_hmacsha512_state,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512_update(
        state: *mut crypto_auth_hmacsha512_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512_final(
        state: *mut crypto_auth_hmacsha512_state,
        out: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_auth_hmacsha512256_bytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_hmacsha512256_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_hmacsha512256(
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512256_verify(
        h: *const ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
pub type crypto_auth_hmacsha512256_state = crypto_auth_hmacsha512_state;
extern "C" {
    pub fn crypto_auth_hmacsha512256_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_hmacsha512256_init(
        state: *mut crypto_auth_hmacsha512256_state,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512256_update(
        state: *mut crypto_auth_hmacsha512256_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512256_final(
        state: *mut crypto_auth_hmacsha512256_state,
        out: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512256_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_auth_bytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_primitive() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn crypto_auth(
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_verify(
        h: *const ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_keygen(k: *mut ::std::os::raw::c_uchar);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_hash_sha256_state {
    pub state: [u32; 8usize],
    pub count: u64,
    pub buf: [u8; 64usize],
}
#[test]
fn bindgen_test_layout_crypto_hash_sha256_state() {
    assert_eq!(
        ::std::mem::size_of::<crypto_hash_sha256_state>(),
        104usize,
        concat!("Size of: ", stringify!(crypto_hash_sha256_state))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_hash_sha256_state>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_hash_sha256_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_hash_sha256_state>())).state as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_hash_sha256_state),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_hash_sha256_state>())).count as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_hash_sha256_state),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_hash_sha256_state>())).buf as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_hash_sha256_state),
            "::",
            stringify!(buf)
        )
    );
}
extern "C" {
    pub fn crypto_hash_sha256_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_hash_sha256_bytes() -> usize;
}
extern "C" {
    pub fn crypto_hash_sha256(
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_hash_sha256_init(state: *mut crypto_hash_sha256_state) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_hash_sha256_update(
        state: *mut crypto_hash_sha256_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_hash_sha256_final(
        state: *mut crypto_hash_sha256_state,
        out: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha256_bytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_hmacsha256_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_hmacsha256(
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha256_verify(
        h: *const ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_auth_hmacsha256_state {
    pub ictx: crypto_hash_sha256_state,
    pub octx: crypto_hash_sha256_state,
}
#[test]
fn bindgen_test_layout_crypto_auth_hmacsha256_state() {
    assert_eq!(
        ::std::mem::size_of::<crypto_auth_hmacsha256_state>(),
        208usize,
        concat!("Size of: ", stringify!(crypto_auth_hmacsha256_state))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_auth_hmacsha256_state>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_auth_hmacsha256_state))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_auth_hmacsha256_state>())).ictx as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_auth_hmacsha256_state),
            "::",
            stringify!(ictx)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_auth_hmacsha256_state>())).octx as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_auth_hmacsha256_state),
            "::",
            stringify!(octx)
        )
    );
}
extern "C" {
    pub fn crypto_auth_hmacsha256_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_hmacsha256_init(
        state: *mut crypto_auth_hmacsha256_state,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha256_update(
        state: *mut crypto_auth_hmacsha256_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha256_final(
        state: *mut crypto_auth_hmacsha256_state,
        out: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha256_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_stream_xsalsa20_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_xsalsa20_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_xsalsa20_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_stream_xsalsa20(
        c: *mut ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_xsalsa20_xor(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_xsalsa20_xor_ic(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        ic: u64,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_xsalsa20_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_seedbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_publickeybytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_secretkeybytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_beforenmbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_macbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_seed_keypair(
        pk: *mut ::std::os::raw::c_uchar,
        sk: *mut ::std::os::raw::c_uchar,
        seed: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_keypair(
        pk: *mut ::std::os::raw::c_uchar,
        sk: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_beforenm(
        k: *mut ::std::os::raw::c_uchar,
        pk: *const ::std::os::raw::c_uchar,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_boxzerobytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_zerobytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        pk: *const ::std::os::raw::c_uchar,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_open(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        pk: *const ::std::os::raw::c_uchar,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_afternm(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_open_afternm(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_seedbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_publickeybytes() -> usize;
}
extern "C" {
    pub fn crypto_box_secretkeybytes() -> usize;
}
extern "C" {
    pub fn crypto_box_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_box_macbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_box_primitive() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn crypto_box_seed_keypair(
        pk: *mut ::std::os::raw::c_uchar,
        sk: *mut ::std::os::raw::c_uchar,
        seed: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_keypair(
        pk: *mut ::std::os::raw::c_uchar,
        sk: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_easy(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        pk: *const ::std::os::raw::c_uchar,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_open_easy(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        pk: *const ::std::os::raw::c_uchar,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_detached(
        c: *mut ::std::os::raw::c_uchar,
        mac: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        pk: *const ::std::os::raw::c_uchar,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_open_detached(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        mac: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        pk: *const ::std::os::raw::c_uchar,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_beforenmbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_beforenm(
        k: *mut ::std::os::raw::c_uchar,
        pk: *const ::std::os::raw::c_uchar,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_easy_afternm(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_open_easy_afternm(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_detached_afternm(
        c: *mut ::std::os::raw::c_uchar,
        mac: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_open_detached_afternm(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        mac: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_sealbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_seal(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        pk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_seal_open(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        pk: *const ::std::os::raw::c_uchar,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_zerobytes() -> usize;
}
extern "C" {
    pub fn crypto_box_boxzerobytes() -> usize;
}
extern "C" {
    pub fn crypto_box(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        pk: *const ::std::os::raw::c_uchar,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_open(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        pk: *const ::std::os::raw::c_uchar,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_afternm(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_open_afternm(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_core_hsalsa20_outputbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_hsalsa20_inputbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_hsalsa20_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_core_hsalsa20_constbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_hsalsa20(
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_core_hchacha20_outputbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_hchacha20_inputbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_hchacha20_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_core_hchacha20_constbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_hchacha20(
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_core_salsa20_outputbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa20_inputbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa20_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa20_constbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa20(
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_core_salsa2012_outputbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa2012_inputbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa2012_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa2012_constbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa2012(
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_core_salsa208_outputbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa208_inputbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa208_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa208_constbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa208(
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_generichash_blake2b_state {
    pub h: [u64; 8usize],
    pub t: [u64; 2usize],
    pub f: [u64; 2usize],
    pub buf: [u8; 256usize],
    pub buflen: usize,
    pub last_node: u8,
    pub __bindgen_padding_0: [u8; 23usize],
}
#[test]
fn bindgen_test_layout_crypto_generichash_blake2b_state() {
    assert_eq!(
        ::std::mem::size_of::<crypto_generichash_blake2b_state>(),
        384usize,
        concat!("Size of: ", stringify!(crypto_generichash_blake2b_state))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_generichash_blake2b_state>())).h as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_generichash_blake2b_state),
            "::",
            stringify!(h)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_generichash_blake2b_state>())).t as *const _ as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_generichash_blake2b_state),
            "::",
            stringify!(t)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_generichash_blake2b_state>())).f as *const _ as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_generichash_blake2b_state),
            "::",
            stringify!(f)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_generichash_blake2b_state>())).buf as *const _ as usize
        },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_generichash_blake2b_state),
            "::",
            stringify!(buf)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_generichash_blake2b_state>())).buflen as *const _ as usize
        },
        352usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_generichash_blake2b_state),
            "::",
            stringify!(buflen)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_generichash_blake2b_state>())).last_node as *const _
                as usize
        },
        360usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_generichash_blake2b_state),
            "::",
            stringify!(last_node)
        )
    );
}
extern "C" {
    pub fn crypto_generichash_blake2b_bytes_min() -> usize;
}
extern "C" {
    pub fn crypto_generichash_blake2b_bytes_max() -> usize;
}
extern "C" {
    pub fn crypto_generichash_blake2b_bytes() -> usize;
}
extern "C" {
    pub fn crypto_generichash_blake2b_keybytes_min() -> usize;
}
extern "C" {
    pub fn crypto_generichash_blake2b_keybytes_max() -> usize;
}
extern "C" {
    pub fn crypto_generichash_blake2b_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_generichash_blake2b_saltbytes() -> usize;
}
extern "C" {
    pub fn crypto_generichash_blake2b_personalbytes() -> usize;
}
extern "C" {
    pub fn crypto_generichash_blake2b_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_generichash_blake2b(
        out: *mut ::std::os::raw::c_uchar,
        outlen: usize,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_generichash_blake2b_salt_personal(
        out: *mut ::std::os::raw::c_uchar,
        outlen: usize,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
        salt: *const ::std::os::raw::c_uchar,
        personal: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_generichash_blake2b_init(
        state: *mut crypto_generichash_blake2b_state,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
        outlen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_generichash_blake2b_init_salt_personal(
        state: *mut crypto_generichash_blake2b_state,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
        outlen: usize,
        salt: *const ::std::os::raw::c_uchar,
        personal: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_generichash_blake2b_update(
        state: *mut crypto_generichash_blake2b_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_generichash_blake2b_final(
        state: *mut crypto_generichash_blake2b_state,
        out: *mut ::std::os::raw::c_uchar,
        outlen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_generichash_blake2b_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_generichash_bytes_min() -> usize;
}
extern "C" {
    pub fn crypto_generichash_bytes_max() -> usize;
}
extern "C" {
    pub fn crypto_generichash_bytes() -> usize;
}
extern "C" {
    pub fn crypto_generichash_keybytes_min() -> usize;
}
extern "C" {
    pub fn crypto_generichash_keybytes_max() -> usize;
}
extern "C" {
    pub fn crypto_generichash_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_generichash_primitive() -> *const ::std::os::raw::c_char;
}
pub type crypto_generichash_state = crypto_generichash_blake2b_state;
extern "C" {
    pub fn crypto_generichash_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_generichash(
        out: *mut ::std::os::raw::c_uchar,
        outlen: usize,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_generichash_init(
        state: *mut crypto_generichash_state,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
        outlen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_generichash_update(
        state: *mut crypto_generichash_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_generichash_final(
        state: *mut crypto_generichash_state,
        out: *mut ::std::os::raw::c_uchar,
        outlen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_generichash_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_hash_bytes() -> usize;
}
extern "C" {
    pub fn crypto_hash(
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_hash_primitive() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn crypto_kdf_blake2b_bytes_min() -> usize;
}
extern "C" {
    pub fn crypto_kdf_blake2b_bytes_max() -> usize;
}
extern "C" {
    pub fn crypto_kdf_blake2b_contextbytes() -> usize;
}
extern "C" {
    pub fn crypto_kdf_blake2b_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_kdf_blake2b_derive_from_key(
        subkey: *mut ::std::os::raw::c_uchar,
        subkey_len: usize,
        subkey_id: u64,
        ctx: *const ::std::os::raw::c_char,
        key: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_kdf_bytes_min() -> usize;
}
extern "C" {
    pub fn crypto_kdf_bytes_max() -> usize;
}
extern "C" {
    pub fn crypto_kdf_contextbytes() -> usize;
}
extern "C" {
    pub fn crypto_kdf_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_kdf_primitive() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn crypto_kdf_derive_from_key(
        subkey: *mut ::std::os::raw::c_uchar,
        subkey_len: usize,
        subkey_id: u64,
        ctx: *const ::std::os::raw::c_char,
        key: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_kdf_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_kx_publickeybytes() -> usize;
}
extern "C" {
    pub fn crypto_kx_secretkeybytes() -> usize;
}
extern "C" {
    pub fn crypto_kx_seedbytes() -> usize;
}
extern "C" {
    pub fn crypto_kx_sessionkeybytes() -> usize;
}
extern "C" {
    pub fn crypto_kx_primitive() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn crypto_kx_seed_keypair(
        pk: *mut ::std::os::raw::c_uchar,
        sk: *mut ::std::os::raw::c_uchar,
        seed: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_kx_keypair(
        pk: *mut ::std::os::raw::c_uchar,
        sk: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_kx_client_session_keys(
        rx: *mut ::std::os::raw::c_uchar,
        tx: *mut ::std::os::raw::c_uchar,
        client_pk: *const ::std::os::raw::c_uchar,
        client_sk: *const ::std::os::raw::c_uchar,
        server_pk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_kx_server_session_keys(
        rx: *mut ::std::os::raw::c_uchar,
        tx: *mut ::std::os::raw::c_uchar,
        server_pk: *const ::std::os::raw::c_uchar,
        server_sk: *const ::std::os::raw::c_uchar,
        client_pk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
pub type __FILE = _IO_FILE;
pub type FILE = _IO_FILE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __mbstate_t {
    pub __count: ::std::os::raw::c_int,
    pub __value: __mbstate_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t__bindgen_ty_1 {
    pub __wch: ::std::os::raw::c_uint,
    pub __wchb: [::std::os::raw::c_char; 4usize],
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout___mbstate_t__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<__mbstate_t__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(__mbstate_t__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<__mbstate_t__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(__mbstate_t__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__mbstate_t__bindgen_ty_1>())).__wch as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t__bindgen_ty_1),
            "::",
            stringify!(__wch)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__mbstate_t__bindgen_ty_1>())).__wchb as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t__bindgen_ty_1),
            "::",
            stringify!(__wchb)
        )
    );
}
#[test]
fn bindgen_test_layout___mbstate_t() {
    assert_eq!(
        ::std::mem::size_of::<__mbstate_t>(),
        8usize,
        concat!("Size of: ", stringify!(__mbstate_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__mbstate_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__mbstate_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__mbstate_t>())).__count as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t),
            "::",
            stringify!(__count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__mbstate_t>())).__value as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t),
            "::",
            stringify!(__value)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
#[test]
fn bindgen_test_layout__G_fpos_t() {
    assert_eq!(
        ::std::mem::size_of::<_G_fpos_t>(),
        16usize,
        concat!("Size of: ", stringify!(_G_fpos_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_G_fpos_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_G_fpos_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_G_fpos_t>())).__pos as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_G_fpos_t),
            "::",
            stringify!(__pos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_G_fpos_t>())).__state as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_G_fpos_t),
            "::",
            stringify!(__state)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos64_t {
    pub __pos: __off64_t,
    pub __state: __mbstate_t,
}
#[test]
fn bindgen_test_layout__G_fpos64_t() {
    assert_eq!(
        ::std::mem::size_of::<_G_fpos64_t>(),
        16usize,
        concat!("Size of: ", stringify!(_G_fpos64_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_G_fpos64_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_G_fpos64_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_G_fpos64_t>())).__pos as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_G_fpos64_t),
            "::",
            stringify!(__pos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_G_fpos64_t>())).__state as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_G_fpos64_t),
            "::",
            stringify!(__state)
        )
    );
}
pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_jump_t {
    _unused: [u8; 0],
}
pub type _IO_lock_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout__IO_marker() {
    assert_eq!(
        ::std::mem::size_of::<_IO_marker>(),
        24usize,
        concat!("Size of: ", stringify!(_IO_marker))
    );
    assert_eq!(
        ::std::mem::align_of::<_IO_marker>(),
        8usize,
        concat!("Alignment of ", stringify!(_IO_marker))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_marker>()))._next as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_marker),
            "::",
            stringify!(_next)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_marker>()))._sbuf as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_marker),
            "::",
            stringify!(_sbuf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_marker>()))._pos as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_marker),
            "::",
            stringify!(_pos)
        )
    );
}
pub const __codecvt_result___codecvt_ok: __codecvt_result = 0;
pub const __codecvt_result___codecvt_partial: __codecvt_result = 1;
pub const __codecvt_result___codecvt_error: __codecvt_result = 2;
pub const __codecvt_result___codecvt_noconv: __codecvt_result = 3;
pub type __codecvt_result = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: ::std::os::raw::c_int,
    pub _IO_read_ptr: *mut ::std::os::raw::c_char,
    pub _IO_read_end: *mut ::std::os::raw::c_char,
    pub _IO_read_base: *mut ::std::os::raw::c_char,
    pub _IO_write_base: *mut ::std::os::raw::c_char,
    pub _IO_write_ptr: *mut ::std::os::raw::c_char,
    pub _IO_write_end: *mut ::std::os::raw::c_char,
    pub _IO_buf_base: *mut ::std::os::raw::c_char,
    pub _IO_buf_end: *mut ::std::os::raw::c_char,
    pub _IO_save_base: *mut ::std::os::raw::c_char,
    pub _IO_backup_base: *mut ::std::os::raw::c_char,
    pub _IO_save_end: *mut ::std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::std::os::raw::c_int,
    pub _flags2: ::std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::std::os::raw::c_ushort,
    pub _vtable_offset: ::std::os::raw::c_schar,
    pub _shortbuf: [::std::os::raw::c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub __pad1: *mut ::std::os::raw::c_void,
    pub __pad2: *mut ::std::os::raw::c_void,
    pub __pad3: *mut ::std::os::raw::c_void,
    pub __pad4: *mut ::std::os::raw::c_void,
    pub __pad5: usize,
    pub _mode: ::std::os::raw::c_int,
    pub _unused2: [::std::os::raw::c_char; 20usize],
}
#[test]
fn bindgen_test_layout__IO_FILE() {
    assert_eq!(
        ::std::mem::size_of::<_IO_FILE>(),
        216usize,
        concat!("Size of: ", stringify!(_IO_FILE))
    );
    assert_eq!(
        ::std::mem::align_of::<_IO_FILE>(),
        8usize,
        concat!("Alignment of ", stringify!(_IO_FILE))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_read_ptr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_read_end as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_read_base as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_write_base as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_write_ptr as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_write_end as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_buf_base as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_buf_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_buf_end as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_buf_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_save_base as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_save_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_backup_base as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_backup_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_save_end as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_save_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._markers as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_markers)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._chain as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_chain)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._fileno as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_fileno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._flags2 as *const _ as usize },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_flags2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._old_offset as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_old_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._cur_column as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_cur_column)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._vtable_offset as *const _ as usize },
        130usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_vtable_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._shortbuf as *const _ as usize },
        131usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_shortbuf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._lock as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_lock)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._offset as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad1 as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad2 as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad3 as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad4 as *const _ as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad4)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad5 as *const _ as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad5)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._mode as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._unused2 as *const _ as usize },
        196usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_unused2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE_plus {
    _unused: [u8; 0],
}
extern "C" {
    #[link_name = "\u{1}_IO_2_1_stdin_"]
    pub static mut _IO_2_1_stdin_: _IO_FILE_plus;
}
extern "C" {
    #[link_name = "\u{1}_IO_2_1_stdout_"]
    pub static mut _IO_2_1_stdout_: _IO_FILE_plus;
}
extern "C" {
    #[link_name = "\u{1}_IO_2_1_stderr_"]
    pub static mut _IO_2_1_stderr_: _IO_FILE_plus;
}
pub type __io_read_fn = ::std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut ::std::os::raw::c_void,
        __buf: *mut ::std::os::raw::c_char,
        __nbytes: usize,
    ) -> __ssize_t,
>;
pub type __io_write_fn = ::std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut ::std::os::raw::c_void,
        __buf: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> __ssize_t,
>;
pub type __io_seek_fn = ::std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut ::std::os::raw::c_void,
        __pos: *mut __off64_t,
        __w: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type __io_close_fn = ::std::option::Option<
    unsafe extern "C" fn(__cookie: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn __underflow(arg1: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __uflow(arg1: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __overflow(arg1: *mut _IO_FILE, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_getc(__fp: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_putc(__c: ::std::os::raw::c_int, __fp: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_feof(__fp: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_ferror(__fp: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_peekc_locked(__fp: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_flockfile(arg1: *mut _IO_FILE);
}
extern "C" {
    pub fn _IO_funlockfile(arg1: *mut _IO_FILE);
}
extern "C" {
    pub fn _IO_ftrylockfile(arg1: *mut _IO_FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_vfscanf(
        arg1: *mut _IO_FILE,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut __va_list_tag,
        arg4: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_vfprintf(
        arg1: *mut _IO_FILE,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _IO_padn(arg1: *mut _IO_FILE, arg2: ::std::os::raw::c_int, arg3: __ssize_t)
        -> __ssize_t;
}
extern "C" {
    pub fn _IO_sgetn(arg1: *mut _IO_FILE, arg2: *mut ::std::os::raw::c_void, arg3: usize) -> usize;
}
extern "C" {
    pub fn _IO_seekoff(
        arg1: *mut _IO_FILE,
        arg2: __off64_t,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> __off64_t;
}
extern "C" {
    pub fn _IO_seekpos(
        arg1: *mut _IO_FILE,
        arg2: __off64_t,
        arg3: ::std::os::raw::c_int,
    ) -> __off64_t;
}
extern "C" {
    pub fn _IO_free_backup_area(arg1: *mut _IO_FILE);
}
pub type fpos_t = _G_fpos_t;
extern "C" {
    #[link_name = "\u{1}stdin"]
    pub static mut stdin: *mut _IO_FILE;
}
extern "C" {
    #[link_name = "\u{1}stdout"]
    pub static mut stdout: *mut _IO_FILE;
}
extern "C" {
    #[link_name = "\u{1}stderr"]
    pub static mut stderr: *mut _IO_FILE;
}
extern "C" {
    pub fn remove(__filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rename(
        __old: *const ::std::os::raw::c_char,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn renameat(
        __oldfd: ::std::os::raw::c_int,
        __old: *const ::std::os::raw::c_char,
        __newfd: ::std::os::raw::c_int,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tmpfile() -> *mut FILE;
}
extern "C" {
    pub fn tmpnam(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tmpnam_r(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tempnam(
        __dir: *const ::std::os::raw::c_char,
        __pfx: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn freopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fdopen(__fd: ::std::os::raw::c_int, __modes: *const ::std::os::raw::c_char)
        -> *mut FILE;
}
extern "C" {
    pub fn fmemopen(
        __s: *mut ::std::os::raw::c_void,
        __len: usize,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn open_memstream(
        __bufloc: *mut *mut ::std::os::raw::c_char,
        __sizeloc: *mut usize,
    ) -> *mut FILE;
}
extern "C" {
    pub fn setbuf(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut ::std::os::raw::c_char,
        __modes: ::std::os::raw::c_int,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setbuffer(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char, __size: usize);
}
extern "C" {
    pub fn setlinebuf(__stream: *mut FILE);
}
extern "C" {
    pub fn fprintf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn printf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sprintf(
        __s: *mut ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfprintf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vprintf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsprintf(
        __s: *mut ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn snprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: usize,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsnprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: usize,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vdprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fscanf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scanf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_fscanf"]
    pub fn fscanf1(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_scanf"]
    pub fn scanf1(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_sscanf"]
    pub fn sscanf1(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfscanf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vscanf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vfscanf"]
    pub fn vfscanf1(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vscanf"]
    pub fn vscanf1(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vsscanf"]
    pub fn vsscanf1(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar_unlocked() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar_unlocked(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getw(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putw(__w: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgets(
        __s: *mut ::std::os::raw::c_char,
        __n: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getline(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn fputs(__s: *const ::std::os::raw::c_char, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn puts(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ungetc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fread(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fwrite(
        __ptr: *const ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __s: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fread_unlocked(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fwrite_unlocked(
        __ptr: *const ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fseek(
        __stream: *mut FILE,
        __off: ::std::os::raw::c_long,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftell(__stream: *mut FILE) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn rewind(__stream: *mut FILE);
}
extern "C" {
    pub fn fseeko(
        __stream: *mut FILE,
        __off: __off_t,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftello(__stream: *mut FILE) -> __off_t;
}
extern "C" {
    pub fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearerr(__stream: *mut FILE);
}
extern "C" {
    pub fn feof(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearerr_unlocked(__stream: *mut FILE);
}
extern "C" {
    pub fn feof_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn perror(__s: *const ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "\u{1}sys_nerr"]
    pub static mut sys_nerr: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}sys_errlist"]
    pub static mut sys_errlist: [*const ::std::os::raw::c_char; 0usize];
}
extern "C" {
    pub fn fileno(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fileno_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn popen(
        __command: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn pclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ctermid(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn flockfile(__stream: *mut FILE);
}
extern "C" {
    pub fn ftrylockfile(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn funlockfile(__stream: *mut FILE);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_onetimeauth_poly1305_state {
    pub opaque: [::std::os::raw::c_uchar; 256usize],
}
#[test]
fn bindgen_test_layout_crypto_onetimeauth_poly1305_state() {
    assert_eq!(
        ::std::mem::size_of::<crypto_onetimeauth_poly1305_state>(),
        256usize,
        concat!("Size of: ", stringify!(crypto_onetimeauth_poly1305_state))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_onetimeauth_poly1305_state>())).opaque as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_onetimeauth_poly1305_state),
            "::",
            stringify!(opaque)
        )
    );
}
extern "C" {
    pub fn crypto_onetimeauth_poly1305_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_onetimeauth_poly1305_bytes() -> usize;
}
extern "C" {
    pub fn crypto_onetimeauth_poly1305_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_onetimeauth_poly1305(
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_poly1305_verify(
        h: *const ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_poly1305_init(
        state: *mut crypto_onetimeauth_poly1305_state,
        key: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_poly1305_update(
        state: *mut crypto_onetimeauth_poly1305_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_poly1305_final(
        state: *mut crypto_onetimeauth_poly1305_state,
        out: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_poly1305_keygen(k: *mut ::std::os::raw::c_uchar);
}
pub type crypto_onetimeauth_state = crypto_onetimeauth_poly1305_state;
extern "C" {
    pub fn crypto_onetimeauth_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_onetimeauth_bytes() -> usize;
}
extern "C" {
    pub fn crypto_onetimeauth_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_onetimeauth_primitive() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn crypto_onetimeauth(
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_verify(
        h: *const ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_init(
        state: *mut crypto_onetimeauth_state,
        key: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_update(
        state: *mut crypto_onetimeauth_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_final(
        state: *mut crypto_onetimeauth_state,
        out: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_pwhash_argon2i_alg_argon2i13() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_bytes_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_bytes_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_passwd_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_passwd_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_saltbytes() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_strbytes() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_strprefix() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_opslimit_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_opslimit_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_memlimit_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_memlimit_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_opslimit_interactive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_memlimit_interactive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_opslimit_moderate() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_memlimit_moderate() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_opslimit_sensitive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_memlimit_sensitive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i(
        out: *mut ::std::os::raw::c_uchar,
        outlen: ::std::os::raw::c_ulonglong,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        salt: *const ::std::os::raw::c_uchar,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
        alg: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_str(
        out: *mut ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_str_verify(
        str: *const ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_str_needs_rehash(
        str: *const ::std::os::raw::c_char,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_alg_argon2id13() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_bytes_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_bytes_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_passwd_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_passwd_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_saltbytes() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_strbytes() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_strprefix() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_opslimit_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_opslimit_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_memlimit_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_memlimit_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_opslimit_interactive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_memlimit_interactive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_opslimit_moderate() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_memlimit_moderate() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_opslimit_sensitive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_memlimit_sensitive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id(
        out: *mut ::std::os::raw::c_uchar,
        outlen: ::std::os::raw::c_ulonglong,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        salt: *const ::std::os::raw::c_uchar,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
        alg: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_str(
        out: *mut ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_str_verify(
        str: *const ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_str_needs_rehash(
        str: *const ::std::os::raw::c_char,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_alg_argon2i13() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_alg_argon2id13() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_alg_default() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_bytes_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_bytes_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_passwd_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_passwd_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_saltbytes() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_strbytes() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_strprefix() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn crypto_pwhash_opslimit_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_opslimit_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_memlimit_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_memlimit_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_opslimit_interactive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_memlimit_interactive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_opslimit_moderate() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_memlimit_moderate() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_opslimit_sensitive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_memlimit_sensitive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash(
        out: *mut ::std::os::raw::c_uchar,
        outlen: ::std::os::raw::c_ulonglong,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        salt: *const ::std::os::raw::c_uchar,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
        alg: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_str(
        out: *mut ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_str_alg(
        out: *mut ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
        alg: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_str_verify(
        str: *const ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_str_needs_rehash(
        str: *const ::std::os::raw::c_char,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_primitive() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn crypto_scalarmult_curve25519_bytes() -> usize;
}
extern "C" {
    pub fn crypto_scalarmult_curve25519_scalarbytes() -> usize;
}
extern "C" {
    pub fn crypto_scalarmult_curve25519(
        q: *mut ::std::os::raw::c_uchar,
        n: *const ::std::os::raw::c_uchar,
        p: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_scalarmult_curve25519_base(
        q: *mut ::std::os::raw::c_uchar,
        n: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_scalarmult_bytes() -> usize;
}
extern "C" {
    pub fn crypto_scalarmult_scalarbytes() -> usize;
}
extern "C" {
    pub fn crypto_scalarmult_primitive() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn crypto_scalarmult_base(
        q: *mut ::std::os::raw::c_uchar,
        n: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_scalarmult(
        q: *mut ::std::os::raw::c_uchar,
        n: *const ::std::os::raw::c_uchar,
        p: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_secretbox_xsalsa20poly1305_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_xsalsa20poly1305_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_xsalsa20poly1305_macbytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_xsalsa20poly1305_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_xsalsa20poly1305(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_secretbox_xsalsa20poly1305_open(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_secretbox_xsalsa20poly1305_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_secretbox_xsalsa20poly1305_boxzerobytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_xsalsa20poly1305_zerobytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_macbytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_primitive() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn crypto_secretbox_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_easy(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_secretbox_open_easy(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_secretbox_detached(
        c: *mut ::std::os::raw::c_uchar,
        mac: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_secretbox_open_detached(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        mac: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_secretbox_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_secretbox_zerobytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_boxzerobytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_secretbox_open(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_chacha20_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_chacha20_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_chacha20_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_stream_chacha20(
        c: *mut ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_chacha20_xor(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_chacha20_xor_ic(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        ic: u64,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_chacha20_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_stream_chacha20_ietf_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_chacha20_ietf_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_chacha20_ietf_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_stream_chacha20_ietf(
        c: *mut ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_chacha20_ietf_xor(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_chacha20_ietf_xor_ic(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        ic: u32,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_chacha20_ietf_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_abytes() -> usize;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_headerbytes() -> usize;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_tag_message() -> ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_tag_push() -> ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_tag_rekey() -> ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_tag_final() -> ::std::os::raw::c_uchar;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct crypto_secretstream_xchacha20poly1305_state {
    pub k: [::std::os::raw::c_uchar; 32usize],
    pub nonce: [::std::os::raw::c_uchar; 12usize],
    pub _pad: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout_crypto_secretstream_xchacha20poly1305_state() {
    assert_eq!(
        ::std::mem::size_of::<crypto_secretstream_xchacha20poly1305_state>(),
        52usize,
        concat!(
            "Size of: ",
            stringify!(crypto_secretstream_xchacha20poly1305_state)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_secretstream_xchacha20poly1305_state>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(crypto_secretstream_xchacha20poly1305_state)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_secretstream_xchacha20poly1305_state>())).k as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_secretstream_xchacha20poly1305_state),
            "::",
            stringify!(k)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_secretstream_xchacha20poly1305_state>())).nonce
                as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_secretstream_xchacha20poly1305_state),
            "::",
            stringify!(nonce)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_secretstream_xchacha20poly1305_state>()))._pad as *const _
                as usize
        },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_secretstream_xchacha20poly1305_state),
            "::",
            stringify!(_pad)
        )
    );
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_init_push(
        state: *mut crypto_secretstream_xchacha20poly1305_state,
        header: *mut ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_push(
        state: *mut crypto_secretstream_xchacha20poly1305_state,
        c: *mut ::std::os::raw::c_uchar,
        clen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        tag: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_init_pull(
        state: *mut crypto_secretstream_xchacha20poly1305_state,
        header: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_pull(
        state: *mut crypto_secretstream_xchacha20poly1305_state,
        m: *mut ::std::os::raw::c_uchar,
        mlen_p: *mut ::std::os::raw::c_ulonglong,
        tag_p: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_rekey(
        state: *mut crypto_secretstream_xchacha20poly1305_state,
    );
}
extern "C" {
    pub fn crypto_shorthash_siphash24_bytes() -> usize;
}
extern "C" {
    pub fn crypto_shorthash_siphash24_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_shorthash_siphash24(
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_shorthash_siphashx24_bytes() -> usize;
}
extern "C" {
    pub fn crypto_shorthash_siphashx24_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_shorthash_siphashx24(
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_shorthash_bytes() -> usize;
}
extern "C" {
    pub fn crypto_shorthash_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_shorthash_primitive() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn crypto_shorthash(
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_shorthash_keygen(k: *mut ::std::os::raw::c_uchar);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_sign_ed25519ph_state {
    pub hs: crypto_hash_sha512_state,
}
#[test]
fn bindgen_test_layout_crypto_sign_ed25519ph_state() {
    assert_eq!(
        ::std::mem::size_of::<crypto_sign_ed25519ph_state>(),
        208usize,
        concat!("Size of: ", stringify!(crypto_sign_ed25519ph_state))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_sign_ed25519ph_state>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_sign_ed25519ph_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_ed25519ph_state>())).hs as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_ed25519ph_state),
            "::",
            stringify!(hs)
        )
    );
}
extern "C" {
    pub fn crypto_sign_ed25519ph_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_ed25519_bytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_ed25519_seedbytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_ed25519_publickeybytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_ed25519_secretkeybytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_ed25519_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_sign_ed25519(
        sm: *mut ::std::os::raw::c_uchar,
        smlen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519_open(
        m: *mut ::std::os::raw::c_uchar,
        mlen_p: *mut ::std::os::raw::c_ulonglong,
        sm: *const ::std::os::raw::c_uchar,
        smlen: ::std::os::raw::c_ulonglong,
        pk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519_detached(
        sig: *mut ::std::os::raw::c_uchar,
        siglen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519_verify_detached(
        sig: *const ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        pk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519_keypair(
        pk: *mut ::std::os::raw::c_uchar,
        sk: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519_seed_keypair(
        pk: *mut ::std::os::raw::c_uchar,
        sk: *mut ::std::os::raw::c_uchar,
        seed: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519_pk_to_curve25519(
        curve25519_pk: *mut ::std::os::raw::c_uchar,
        ed25519_pk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519_sk_to_curve25519(
        curve25519_sk: *mut ::std::os::raw::c_uchar,
        ed25519_sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519_sk_to_seed(
        seed: *mut ::std::os::raw::c_uchar,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519_sk_to_pk(
        pk: *mut ::std::os::raw::c_uchar,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519ph_init(
        state: *mut crypto_sign_ed25519ph_state,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519ph_update(
        state: *mut crypto_sign_ed25519ph_state,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519ph_final_create(
        state: *mut crypto_sign_ed25519ph_state,
        sig: *mut ::std::os::raw::c_uchar,
        siglen_p: *mut ::std::os::raw::c_ulonglong,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519ph_final_verify(
        state: *mut crypto_sign_ed25519ph_state,
        sig: *mut ::std::os::raw::c_uchar,
        pk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
pub type crypto_sign_state = crypto_sign_ed25519ph_state;
extern "C" {
    pub fn crypto_sign_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_bytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_seedbytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_publickeybytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_secretkeybytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_sign_primitive() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn crypto_sign_seed_keypair(
        pk: *mut ::std::os::raw::c_uchar,
        sk: *mut ::std::os::raw::c_uchar,
        seed: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_keypair(
        pk: *mut ::std::os::raw::c_uchar,
        sk: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign(
        sm: *mut ::std::os::raw::c_uchar,
        smlen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_open(
        m: *mut ::std::os::raw::c_uchar,
        mlen_p: *mut ::std::os::raw::c_ulonglong,
        sm: *const ::std::os::raw::c_uchar,
        smlen: ::std::os::raw::c_ulonglong,
        pk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_detached(
        sig: *mut ::std::os::raw::c_uchar,
        siglen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_verify_detached(
        sig: *const ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        pk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_init(state: *mut crypto_sign_state) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_update(
        state: *mut crypto_sign_state,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_final_create(
        state: *mut crypto_sign_state,
        sig: *mut ::std::os::raw::c_uchar,
        siglen_p: *mut ::std::os::raw::c_ulonglong,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_final_verify(
        state: *mut crypto_sign_state,
        sig: *mut ::std::os::raw::c_uchar,
        pk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_stream_primitive() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn crypto_stream(
        c: *mut ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_xor(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_stream_salsa20_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_salsa20_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_salsa20_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_stream_salsa20(
        c: *mut ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_salsa20_xor(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_salsa20_xor_ic(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        ic: u64,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_salsa20_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_verify_16_bytes() -> usize;
}
extern "C" {
    pub fn crypto_verify_16(
        x: *const ::std::os::raw::c_uchar,
        y: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_verify_32_bytes() -> usize;
}
extern "C" {
    pub fn crypto_verify_32(
        x: *const ::std::os::raw::c_uchar,
        y: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_verify_64_bytes() -> usize;
}
extern "C" {
    pub fn crypto_verify_64(
        x: *const ::std::os::raw::c_uchar,
        y: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct randombytes_implementation {
    pub implementation_name:
        ::std::option::Option<unsafe extern "C" fn() -> *const ::std::os::raw::c_char>,
    pub random: ::std::option::Option<unsafe extern "C" fn() -> u32>,
    pub stir: ::std::option::Option<unsafe extern "C" fn()>,
    pub uniform: ::std::option::Option<unsafe extern "C" fn(upper_bound: u32) -> u32>,
    pub buf:
        ::std::option::Option<unsafe extern "C" fn(buf: *mut ::std::os::raw::c_void, size: usize)>,
    pub close: ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
}
#[test]
fn bindgen_test_layout_randombytes_implementation() {
    assert_eq!(
        ::std::mem::size_of::<randombytes_implementation>(),
        48usize,
        concat!("Size of: ", stringify!(randombytes_implementation))
    );
    assert_eq!(
        ::std::mem::align_of::<randombytes_implementation>(),
        8usize,
        concat!("Alignment of ", stringify!(randombytes_implementation))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<randombytes_implementation>())).implementation_name as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(randombytes_implementation),
            "::",
            stringify!(implementation_name)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<randombytes_implementation>())).random as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(randombytes_implementation),
            "::",
            stringify!(random)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<randombytes_implementation>())).stir as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(randombytes_implementation),
            "::",
            stringify!(stir)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<randombytes_implementation>())).uniform as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(randombytes_implementation),
            "::",
            stringify!(uniform)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<randombytes_implementation>())).buf as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(randombytes_implementation),
            "::",
            stringify!(buf)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<randombytes_implementation>())).close as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(randombytes_implementation),
            "::",
            stringify!(close)
        )
    );
}
extern "C" {
    pub fn randombytes_seedbytes() -> usize;
}
extern "C" {
    pub fn randombytes_buf(buf: *mut ::std::os::raw::c_void, size: usize);
}
extern "C" {
    pub fn randombytes_buf_deterministic(
        buf: *mut ::std::os::raw::c_void,
        size: usize,
        seed: *const ::std::os::raw::c_uchar,
    );
}
extern "C" {
    pub fn randombytes_random() -> u32;
}
extern "C" {
    pub fn randombytes_uniform(upper_bound: u32) -> u32;
}
extern "C" {
    pub fn randombytes_stir();
}
extern "C" {
    pub fn randombytes_close() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn randombytes_set_implementation(
        impl_: *mut randombytes_implementation,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn randombytes_implementation_name() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn randombytes(buf: *mut ::std::os::raw::c_uchar, buf_len: ::std::os::raw::c_ulonglong);
}
extern "C" {
    #[link_name = "\u{1}randombytes_salsa20_implementation"]
    pub static mut randombytes_salsa20_implementation: randombytes_implementation;
}
extern "C" {
    #[link_name = "\u{1}randombytes_sysrandom_implementation"]
    pub static mut randombytes_sysrandom_implementation: randombytes_implementation;
}
extern "C" {
    pub fn sodium_runtime_has_neon() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_sse2() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_sse3() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_ssse3() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_sse41() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_avx() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_avx2() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_avx512f() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_pclmul() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_aesni() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_rdrand() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _sodium_runtime_get_cpu_features() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_memzero(pnt: *mut ::std::os::raw::c_void, len: usize);
}
extern "C" {
    pub fn sodium_stackzero(len: usize);
}
extern "C" {
    pub fn sodium_memcmp(
        b1_: *const ::std::os::raw::c_void,
        b2_: *const ::std::os::raw::c_void,
        len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_compare(
        b1_: *const ::std::os::raw::c_uchar,
        b2_: *const ::std::os::raw::c_uchar,
        len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_is_zero(n: *const ::std::os::raw::c_uchar, nlen: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_increment(n: *mut ::std::os::raw::c_uchar, nlen: usize);
}
extern "C" {
    pub fn sodium_add(
        a: *mut ::std::os::raw::c_uchar,
        b: *const ::std::os::raw::c_uchar,
        len: usize,
    );
}
extern "C" {
    pub fn sodium_bin2hex(
        hex: *mut ::std::os::raw::c_char,
        hex_maxlen: usize,
        bin: *const ::std::os::raw::c_uchar,
        bin_len: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn sodium_hex2bin(
        bin: *mut ::std::os::raw::c_uchar,
        bin_maxlen: usize,
        hex: *const ::std::os::raw::c_char,
        hex_len: usize,
        ignore: *const ::std::os::raw::c_char,
        bin_len: *mut usize,
        hex_end: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_base64_encoded_len(bin_len: usize, variant: ::std::os::raw::c_int) -> usize;
}
extern "C" {
    pub fn sodium_bin2base64(
        b64: *mut ::std::os::raw::c_char,
        b64_maxlen: usize,
        bin: *const ::std::os::raw::c_uchar,
        bin_len: usize,
        variant: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn sodium_base642bin(
        bin: *mut ::std::os::raw::c_uchar,
        bin_maxlen: usize,
        b64: *const ::std::os::raw::c_char,
        b64_len: usize,
        ignore: *const ::std::os::raw::c_char,
        bin_len: *mut usize,
        b64_end: *mut *const ::std::os::raw::c_char,
        variant: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_mlock(addr: *mut ::std::os::raw::c_void, len: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_munlock(addr: *mut ::std::os::raw::c_void, len: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_malloc(size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn sodium_allocarray(count: usize, size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn sodium_free(ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn sodium_mprotect_noaccess(ptr: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_mprotect_readonly(ptr: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_mprotect_readwrite(ptr: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_pad(
        padded_buflen_p: *mut usize,
        buf: *mut ::std::os::raw::c_uchar,
        unpadded_buflen: usize,
        blocksize: usize,
        max_buflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_unpad(
        unpadded_buflen_p: *mut usize,
        buf: *const ::std::os::raw::c_uchar,
        padded_buflen: usize,
        blocksize: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _sodium_alloc_init() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_xchacha20_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_xchacha20_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_xchacha20_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_stream_xchacha20(
        c: *mut ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_xchacha20_xor(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_xchacha20_xor_ic(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        ic: u64,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_xchacha20_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_seedbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_publickeybytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_secretkeybytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_beforenmbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_macbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_seed_keypair(
        pk: *mut ::std::os::raw::c_uchar,
        sk: *mut ::std::os::raw::c_uchar,
        seed: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_keypair(
        pk: *mut ::std::os::raw::c_uchar,
        sk: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_easy(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        pk: *const ::std::os::raw::c_uchar,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_open_easy(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        pk: *const ::std::os::raw::c_uchar,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_detached(
        c: *mut ::std::os::raw::c_uchar,
        mac: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        pk: *const ::std::os::raw::c_uchar,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_open_detached(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        mac: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        pk: *const ::std::os::raw::c_uchar,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_beforenm(
        k: *mut ::std::os::raw::c_uchar,
        pk: *const ::std::os::raw::c_uchar,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_easy_afternm(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_open_easy_afternm(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_detached_afternm(
        c: *mut ::std::os::raw::c_uchar,
        mac: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_open_detached_afternm(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        mac: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_sealbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_seal(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        pk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_seal_open(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        pk: *const ::std::os::raw::c_uchar,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_core_ed25519_bytes() -> usize;
}
extern "C" {
    pub fn crypto_core_ed25519_uniformbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_ed25519_is_valid_point(
        p: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_core_ed25519_add(
        r: *mut ::std::os::raw::c_uchar,
        p: *const ::std::os::raw::c_uchar,
        q: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_core_ed25519_sub(
        r: *mut ::std::os::raw::c_uchar,
        p: *const ::std::os::raw::c_uchar,
        q: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_core_ed25519_from_uniform(
        p: *mut ::std::os::raw::c_uchar,
        r: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_scalarmult_ed25519_bytes() -> usize;
}
extern "C" {
    pub fn crypto_scalarmult_ed25519_scalarbytes() -> usize;
}
extern "C" {
    pub fn crypto_scalarmult_ed25519(
        q: *mut ::std::os::raw::c_uchar,
        n: *const ::std::os::raw::c_uchar,
        p: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_scalarmult_ed25519_base(
        q: *mut ::std::os::raw::c_uchar,
        n: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_secretbox_xchacha20poly1305_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_xchacha20poly1305_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_xchacha20poly1305_macbytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_xchacha20poly1305_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_xchacha20poly1305_easy(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_secretbox_xchacha20poly1305_open_easy(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_secretbox_xchacha20poly1305_detached(
        c: *mut ::std::os::raw::c_uchar,
        mac: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_secretbox_xchacha20poly1305_open_detached(
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        mac: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_bytes_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_bytes_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_passwd_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_passwd_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_saltbytes() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_strbytes() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_strprefix() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_opslimit_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_opslimit_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_memlimit_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_memlimit_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_opslimit_interactive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_memlimit_interactive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_opslimit_sensitive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_memlimit_sensitive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256(
        out: *mut ::std::os::raw::c_uchar,
        outlen: ::std::os::raw::c_ulonglong,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        salt: *const ::std::os::raw::c_uchar,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_str(
        out: *mut ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_str_verify(
        str: *const ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_ll(
        passwd: *const u8,
        passwdlen: usize,
        salt: *const u8,
        saltlen: usize,
        N: u64,
        r: u32,
        p: u32,
        buf: *mut u8,
        buflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_str_needs_rehash(
        str: *const ::std::os::raw::c_char,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_salsa2012_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_salsa2012_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_salsa2012_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_stream_salsa2012(
        c: *mut ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_salsa2012_xor(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_salsa2012_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_stream_salsa208_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_salsa208_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_salsa208_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_stream_salsa208(
        c: *mut ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_salsa208_xor(
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_stream_salsa208_keygen(k: *mut ::std::os::raw::c_uchar);
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout___va_list_tag() {
    assert_eq!(
        ::std::mem::size_of::<__va_list_tag>(),
        24usize,
        concat!("Size of: ", stringify!(__va_list_tag))
    );
    assert_eq!(
        ::std::mem::align_of::<__va_list_tag>(),
        8usize,
        concat!("Alignment of ", stringify!(__va_list_tag))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).gp_offset as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(gp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).fp_offset as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(fp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).overflow_arg_area as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(overflow_arg_area)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).reg_save_area as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(reg_save_area)
        )
    );
}
