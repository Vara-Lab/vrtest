use vrtest::{
    runtime::*
};

const SIGNER: u64 = 1;
const VAL_1_STASH: u64 = 10;
const VAL_1_STASH_AUTH_ID: u64 = 11;

#[test]
pub fn test_runtime_time_functions() {
    new_test_ext(vec![SIGNER]).execute_with(|| {
        // Block duration 1000 ms
        assert_eq!(
            block_in_ms(),
            1_000
        );

        // Sessions per era =  5
        assert_eq!(
            sessions_per_era(),
            6
        );

        // Session duration in blocks = 250
        assert_eq!(
            session_duration_in_blocks(),
            250
        );

        // Era duration in blocks
        assert_eq!(
            era_duration_in_blocks(),
            1_500
        );

        // Era duration in ms
        assert_eq!(
            era_duration_ms(),
            1_500_000
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

        // block 251 - session 1 ----------------------------

       { run_for_n_blocks( 
            250, 
            None
        );

        assert_eq!(
            current_block(),
            251
        );

        assert_eq!(
            current_session_index(),
            1
        );

        assert_eq!(
            current_era(),
            0
        );}

        // block 501 - session 2 ----------------------------

        {run_for_n_blocks( 
            250, 
            None
        );

        assert_eq!(
            current_block(),
            501
        );

        assert_eq!(
            current_session_index(),
            2
        );

        assert_eq!(
            current_era(),
            0
        );}

        // block 751 - session 3 ----------------------------

       { run_for_n_blocks( 
            250, 
            None
        );

        assert_eq!(
            current_block(),
            751
        );

        assert_eq!(
            current_session_index(),
            3
        );

        assert_eq!(
            current_era(),
            0
        );}

        // block 1001 - session 4 ----------------------------

        run_for_n_blocks( 
            250, 
            None
        );

        assert_eq!(
            current_block(),
            1001
        );

        assert_eq!(
            current_session_index(),
            4
        );

        assert_eq!(
            current_era(),
            0
        );

        // block 1251 - session 5 ----------------------------

        run_for_n_blocks( 
            250, 
            None
        );

        assert_eq!(
            current_block(),
            1251
        );

        assert_eq!(
            current_session_index(),
            5
        );

        assert_eq!(
            current_era(),
            1
        );

        // block 1501 - session 6 ----------------------------

        run_for_n_blocks( 
            250, 
            None
        );

        assert_eq!(
            current_block(),
            1501
        );

        assert_eq!(
            current_session_index(),
            6
        );

        assert_eq!(
            current_era(),
            1
        );

        // block 1751 - session 7 ----------------------------

        run_for_n_blocks( 
            250, 
            None
        );

        assert_eq!(
            current_block(),
            1751
        );

        assert_eq!(
            current_session_index(),
            7
        );

        assert_eq!(
            current_era(),
            1
        );

        // block 2001 - session 8 ----------------------------

        run_for_n_blocks( 
            250, 
            None
        );

        assert_eq!(
            current_block(),
            2001
        );

        assert_eq!(
            current_session_index(),
            8
        );

        assert_eq!(
            current_era(),
            1
        );

        // block 2251 - session 9 ----------------------------

        run_for_n_blocks( 
            250, 
            None
        );

        assert_eq!(
            current_block(),
            2251
        );

        assert_eq!(
            current_session_index(),
            9
        );

        assert_eq!(
            current_era(),
            1
        );

        // block 2501 - session 10 ----------------------------

        run_for_n_blocks( 
            250, 
            None
        );

        assert_eq!(
            current_block(),
            2501
        );

        assert_eq!(
            current_session_index(),
            10
        );

        assert_eq!(
            current_era(),
            1
        );

        // block 2751 - session 11 ----------------------------

        run_for_n_blocks( 
            250, 
            None
        );

        assert_eq!(
            current_block(),
            2751
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
            1250, 
            None
        );

        assert_eq!(
            current_block(),
            4001
        );

        assert_eq!(
            current_session_index(),
            16
        );

        assert_eq!(
            current_era(),
            2
        );

        // block 4251 - session 17 ----------------------------

        run_for_n_blocks( 
            250, 
            None
        );

        assert_eq!(
            current_block(),
            4251
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


