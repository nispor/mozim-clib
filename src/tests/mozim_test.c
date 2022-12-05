// SPDX-License-Identifier: Apache-2.0

#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <mozim.h>

#define WAIT_TIME               10
#define PROCESS_LOOP_COUNT      10
#define TEST_NIC                "eth1"

int process(struct mozim_dhcpv4_client *client) {
    int rc = EXIT_SUCCESS;
    uint32_t ret = MOZIM_PASS;
    uint64_t *events = NULL;
    uint64_t event_count = 0;
    uint64_t i = 0;
    char *log = NULL;
    char *err_kind = NULL;
    char *err_msg = NULL;
    struct mozim_dhcpv4_lease *lease = NULL;

    ret = mozim_dhcpv4_client_poll(client, WAIT_TIME, &events, &event_count,
                                   &log, &err_kind, &err_msg);
    printf("Log %s\n", log);
    mozim_cstring_free(log);

    if (ret != MOZIM_PASS) {
        printf("Error: %s: %s\n", err_kind, err_msg);
        rc = EXIT_FAILURE;
        mozim_cstring_free(err_kind);
        mozim_cstring_free(err_msg);
        goto out;
    }

    for (i=0; i < event_count; ++i) {
        ret = mozim_dhcpv4_client_process(client, events[i], &lease, &log,
                                          &err_kind, &err_msg);
        printf("Log %s\n", log);
        mozim_cstring_free(log);
        if (ret != MOZIM_PASS) {
            printf("Error: %s: %s\n", err_kind, err_msg);
            rc = EXIT_FAILURE;
            mozim_cstring_free(err_kind);
            mozim_cstring_free(err_msg);
            goto out;
        } else {
            if (lease != NULL) {
                printf("Got lease: t1 %u t2 %u ip %u\n",
                       mozim_dhcpv4_lease_get_t1(lease),
                       mozim_dhcpv4_lease_get_t2(lease),
                       mozim_dhcpv4_lease_get_yiaddr(lease));
                mozim_dhcpv4_lease_free(lease);
            }
        }
    }

out:
    mozim_events_free(events, event_count);
    return rc;
}

int main(void) {
    int rc = EXIT_SUCCESS;
    uint32_t ret = MOZIM_PASS;
    struct mozim_dhcpv4_config *config = NULL;
    struct mozim_dhcpv4_client *client = NULL;
    char *err_kind = NULL;
    char *err_msg = NULL;
    char *log = NULL;
    int i = 0;

    if (mozim_dhcpv4_config_new(&config, TEST_NIC) != MOZIM_PASS) {
        printf("Error: failed to create `mozim_dhcpv4_config` for %s\n",
               TEST_NIC);
        rc = EXIT_FAILURE;
        goto out;
    }

    ret = mozim_dhcpv4_client_init(&client, config, &log, &err_kind, &err_msg);
    printf("Log %s\n", log);

    if (ret != MOZIM_PASS) {
        printf("Error: %s: %s\n", err_kind, err_msg);
        rc = EXIT_FAILURE;
        goto out;
    }

    for (i = 0; i < PROCESS_LOOP_COUNT; ++i) {
        if (process(client) != EXIT_SUCCESS) {
            goto out;
        }
    }

 out:
    mozim_cstring_free(err_kind);
    mozim_cstring_free(err_msg);
    mozim_cstring_free(log);
    mozim_dhcpv4_client_free(client);
    mozim_dhcpv4_config_free(config);
    exit(rc);
}
