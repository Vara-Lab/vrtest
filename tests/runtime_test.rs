use vrtest::{
    runtime::*
};

const SIGNER: u64 = 1;
const VAL_1_STASH: u64 = 10;
const VAL_1_STASH_AUTH_ID: u64 = 11;

#[test]
pub fn test_runtime_time_functions() {
    new_test_ext(vec![SIGNER]).execute_with(|| {
        // Block duration 3000 ms
        assert_eq!(
            block_in_ms(),
            3_000
        );

        // Sessions per era = 6
        assert_eq!(
            sessions_per_era(),
            6
        );

        // Session duration in blocks = 2_400 (2 hours)
        assert_eq!(
            session_duration_in_blocks(),
            2_400
        );

        // Era duration in blocks (12 hours)
        assert_eq!(
            era_duration_in_blocks(),
            14_400
        );

        // Era duration in ms
        assert_eq!(
            era_duration_ms(),
            43_200_000
        )
        
    });
}

#[test]
pub fn test_correct_block_count_for_sessions_and_eras() {
    let authorities = vec![
        (VAL_1_STASH, VAL_1_STASH_AUTH_ID),
    ];

    new_test_ext_with_authorities_and_sessions(vec![SIGNER], authorities).execute_with(|| {
        let session_index = current_session_index();

        assert_eq!(
            session_index,
            0
        );

        assert_eq!(
            current_block(),
            1
        );

        assert_eq!(
            current_era(),
            0
        );

        // block 2_400 - session 1 ----------------------------

       { run_for_n_blocks( 
            2_400, 
            None
        );

        assert_eq!(
            current_block(),
            2_401
        );

        assert_eq!(
            current_session_index(),
            1
        );

        assert_eq!(
            current_era(),
            0
        );}

        // block 4_800 - session 2 ----------------------------

        {run_for_n_blocks( 
            2_400, 
            None
        );

        assert_eq!(
            current_block(),
            4_801
        );

        assert_eq!(
            current_session_index(),
            2
        );

        assert_eq!(
            current_era(),
            0
        );}

        // block 7_200 - session 3 ----------------------------

       { run_for_n_blocks( 
            2_400, 
            None
        );

        assert_eq!(
            current_block(),
            7_201
        );

        assert_eq!(
            current_session_index(),
            3
        );

        assert_eq!(
            current_era(),
            0
        );}

        // block 9_600 - session 4 ----------------------------

        run_for_n_blocks( 
            2_400, 
            None
        );

        assert_eq!(
            current_block(),
            9_601
        );

        assert_eq!(
            current_session_index(),
            4
        );

        assert_eq!(
            current_era(),
            0
        );

        // block 12000 - session 5 ----------------------------

        run_for_n_blocks( 
            2_400, 
            None
        );

        assert_eq!(
            current_block(),
            12_001
        );

        assert_eq!(
            current_session_index(),
            5
        );

        assert_eq!(
            current_era(),
            1
        );

        // block 14_400 - session 6 ----------------------------

        run_for_n_blocks( 
            2_400, 
            None
        );

        assert_eq!(
            current_block(),
            14_401
        );

        assert_eq!(
            current_session_index(),
            6
        );

        assert_eq!(
            current_era(),
            1
        );

        // block 16_800 - session 7 ----------------------------

        run_for_n_blocks( 
            2400, 
            None
        );

        assert_eq!(
            current_block(),
            16_801
        );

        assert_eq!(
            current_session_index(),
            7
        );

        assert_eq!(
            current_era(),
            1
        );

        // block 19_200 - session 8 ----------------------------

        run_for_n_blocks( 
            2_400, 
            None
        );

        assert_eq!(
            current_block(),
            19_201
        );

        assert_eq!(
            current_session_index(),
            8
        );

        assert_eq!(
            current_era(),
            1
        );

        // block 21_600 - session 9 ----------------------------

        run_for_n_blocks( 
            2_400, 
            None
        );

        assert_eq!(
            current_block(),
            21_601
        );

        assert_eq!(
            current_session_index(),
            9
        );

        assert_eq!(
            current_era(),
            1
        );

        // block 24_000 - session 10 ----------------------------

        run_for_n_blocks( 
            2_400, 
            None
        );

        assert_eq!(
            current_block(),
            24_001
        );

        assert_eq!(
            current_session_index(),
            10
        );

        assert_eq!(
            current_era(),
            1
        );

        // block 26_400 - session 11 ----------------------------

        run_for_n_blocks( 
            2400, 
            None
        );

        assert_eq!(
            current_block(),
            26_401
        );

        assert_eq!(
            current_session_index(),
            11
        );

        assert_eq!(
            current_era(),
            2
        );

        run_for_n_blocks( // Run 5 sessions
            12_000, 
            None
        );

        assert_eq!(
            current_block(),
            38_401
        );

        assert_eq!(
            current_session_index(),
            16
        );

        assert_eq!(
            current_era(),
            2
        );

        // block 40_800 - session 17 ----------------------------

        run_for_n_blocks( 
            2_400, 
            None
        );

        assert_eq!(
            current_block(),
            40_801
        );

        assert_eq!(
            current_session_index(),
            17
        );

        assert_eq!(
            current_era(),
            3
        );
    });
}


