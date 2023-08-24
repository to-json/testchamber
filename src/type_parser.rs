

pub fn type_parser(t: &str) {
    dbg!(t);


}

enum Type {
    Char,
    Double,
    Float,
    Int,
    Void,
    Struct,
    Union,
    // the c parser calls these types, tempted to call them modifiers, or,
    // register types for each rational pairing
    Signed,
    Unsigned,
}

enum Qualifier {
    Const,
    Volatile // This is never used for syscalls, but, may be handy later
}














/// I don't know what this will look like, but type `bpf_attr` is a union of 4
/// structs that are only determinable by runtime context. there will probably
/// be an enum of the structs, and a special handler for bpf syscalls, but,
/// storing a bunch of useful context here for now
///
/// * `cmd`: int, specifies which bpf command is being called, which implicitly
///    specifies which struct to use
/// * `bpf_attr`: this will actually be a pointer to the particular struct
///
/// there is also a size attribute, unsure if i'll need that
fn _handle_bpf_attr(_cmd: i32, _bpf_attr: i32) {}

// this comment is a large chunk of bpf.h from the linux kernel v4.9
// similar code in the 6-series is unchanged. just some reference material
//
// enum bpf_cmd {
// 	BPF_MAP_CREATE,
// 	BPF_MAP_LOOKUP_ELEM,
// 	BPF_MAP_UPDATE_ELEM,
// 	BPF_MAP_DELETE_ELEM,
// 	BPF_MAP_GET_NEXT_KEY,
// 	BPF_PROG_LOAD,
// 	BPF_OBJ_PIN,
// 	BPF_OBJ_GET,
// };
//
// enum bpf_map_type {
// 	BPF_MAP_TYPE_UNSPEC,
// 	BPF_MAP_TYPE_HASH,
// 	BPF_MAP_TYPE_ARRAY,
// 	BPF_MAP_TYPE_PROG_ARRAY,
// 	BPF_MAP_TYPE_PERF_EVENT_ARRAY,
// 	BPF_MAP_TYPE_PERCPU_HASH,
// 	BPF_MAP_TYPE_PERCPU_ARRAY,
// 	BPF_MAP_TYPE_STACK_TRACE,
// 	BPF_MAP_TYPE_CGROUP_ARRAY,
// };
//
// enum bpf_prog_type {
// 	BPF_PROG_TYPE_UNSPEC,
// 	BPF_PROG_TYPE_SOCKET_FILTER,
// 	BPF_PROG_TYPE_KPROBE,
// 	BPF_PROG_TYPE_SCHED_CLS,
// 	BPF_PROG_TYPE_SCHED_ACT,
// 	BPF_PROG_TYPE_TRACEPOINT,
// 	BPF_PROG_TYPE_XDP,
// 	BPF_PROG_TYPE_PERF_EVENT,
// };
//
// #define BPF_PSEUDO_MAP_FD	1
//
// /* flags for BPF_MAP_UPDATE_ELEM command */
// #define BPF_ANY		0 /* create new element or update existing */
// #define BPF_NOEXIST	1 /* create new element if it didn't exist */
// #define BPF_EXIST	2 /* update existing element */
//
// #define BPF_F_NO_PREALLOC	(1U << 0)
//
// union bpf_attr {
// 	struct { /* anonymous struct used by BPF_MAP_CREATE command */
// 		__u32	map_type;	/* one of enum bpf_map_type */
// 		__u32	key_size;	/* size of key in bytes */
// 		__u32	value_size;	/* size of value in bytes */
// 		__u32	max_entries;	/* max number of entries in a map */
// 		__u32	map_flags;	/* prealloc or not */
// 	};
//
// 	struct { /* anonymous struct used by BPF_MAP_*_ELEM commands */
// 		__u32		map_fd;
// 		__aligned_u64	key;
// 		union {
// 			__aligned_u64 value;
// 			__aligned_u64 next_key;
// 		};
// 		__u64		flags;
// 	};
//
// 	struct { /* anonymous struct used by BPF_PROG_LOAD command */
// 		__u32		prog_type;	/* one of enum bpf_prog_type */
// 		__u32		insn_cnt;
// 		__aligned_u64	insns;
// 		__aligned_u64	license;
// 		__u32		log_level;	/* verbosity level of verifier */
// 		__u32		log_size;	/* size of user buffer */
// 		__aligned_u64	log_buf;	/* user supplied buffer */
// 		__u32		kern_version;	/* checked when prog_type=kprobe */
// 	};
//
// 	struct { /* anonymous struct used by BPF_OBJ_* commands */
// 		__aligned_u64	pathname;
// 		__u32		bpf_fd;
// 	};
// } __attribute__((aligned(8)));
